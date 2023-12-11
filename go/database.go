package main

import (
	"encoding/json"
	"log"
	"os"
)

type Database struct {
	Users     []User     `json:"users"`
	Musics    []Music    `json:"musics"`
	Playlists []Playlist `json:"playlists"`
}

var db Database

func LoadDatabase(filename string) {
	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatalf("cannot read database from json file: %s", err)
	}

	err = json.Unmarshal(data, &db)
	if err != nil {
		log.Fatalf("cannot unmarshal json file into Database struct: %s", err)
	}
}

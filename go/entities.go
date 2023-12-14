package main

import api "github.com/wilmacedo/streaming/go/proto"

type User struct {
	Id   string
	Name string
	Age  uint64
}

type Music struct {
	Id         string
	Name       string
	Author     string
	PlaylistId string
}

type Playlist struct {
	Id     string
	Name   string
	UserId string
}

func (u *User) ToProto() *api.User {
	return &api.User{
		Id:   u.Id,
		Name: u.Name,
		Age:  u.Age,
	}
}

func (m *Music) ToProto() *api.Music {
	return &api.Music{
		Id:         m.Id,
		Name:       m.Name,
		Author:     m.Author,
		PlaylistId: m.PlaylistId,
	}
}

func (p *Playlist) ToProto() *api.Playlist {
	return &api.Playlist{
		Id:     p.Id,
		Name:   p.Name,
		UserId: p.UserId,
	}
}

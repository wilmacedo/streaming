package main

import (
	"context"
	"errors"

	api "github.com/wilmacedo/streaming/go/proto"
	"google.golang.org/protobuf/types/known/emptypb"
)

type APIServer struct {
	api.UnimplementedAPIServer
}

func (s APIServer) FindUser(ctx context.Context, req *api.FindRequest) (*api.User, error) {
	for _, user := range db.Users {
		if user.Id == req.Id {
			return user.ToProto(), nil
		}
	}

	return nil, errors.New("user not found")
}

func (s APIServer) StreamUsers(_ *emptypb.Empty, stream api.API_StreamUsersServer) error {
	for _, user := range db.Users {
		if err := stream.Send(user.ToProto()); err != nil {
			return err
		}
	}
	return nil
}

func (s APIServer) ListUsers(_ context.Context, _ *emptypb.Empty) (*api.ListUserResponse, error) {
	var res api.ListUserResponse
	for _, user := range db.Users {
		res.Users = append(res.Users, user.ToProto())
	}
	return &res, nil
}

func (s APIServer) FindMusic(ctx context.Context, req *api.FindRequest) (*api.Music, error) {
	for _, music := range db.Musics {
		if music.Id == req.Id {
			return music.ToProto(), nil
		}
	}
	return nil, errors.New("music not found")
}

func (s APIServer) StreamMusics(_ *emptypb.Empty, stream api.API_StreamMusicsServer) error {
	for _, music := range db.Musics {
		if err := stream.Send(music.ToProto()); err != nil {
			return err
		}
	}
	return nil
}

func (s APIServer) ListMusics(context.Context, *emptypb.Empty) (*api.ListMusicResponse, error) {
	var res api.ListMusicResponse
	for _, music := range db.Musics {
		res.Musics = append(res.Musics, music.ToProto())
	}
	return &res, nil
}

func (s APIServer) FindPlaylist(ctx context.Context, req *api.FindRequest) (*api.Playlist, error) {
	for _, playlist := range db.Playlists {
		if playlist.Id == req.Id {
			return playlist.ToProto(), nil
		}
	}
	return nil, errors.New("playlist not found")
}

func (s APIServer) StreamPlaylists(_ *emptypb.Empty, stream api.API_StreamPlaylistsServer) error {
	for _, playlist := range db.Playlists {
		if err := stream.Send(playlist.ToProto()); err != nil {
			return err
		}
	}
	return nil
}

func (s APIServer) ListPlaylists(context.Context, *emptypb.Empty) (*api.ListPlaylistResponse, error) {
	var res api.ListPlaylistResponse
	for _, playlist := range db.Playlists {
		res.Playlists = append(res.Playlists, playlist.ToProto())
	}
	return &res, nil
}

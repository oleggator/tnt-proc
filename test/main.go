package main

import (
	"github.com/google/uuid"
	"github.com/tarantool/go-tarantool"
	"log"
)

type Args struct {
	_msgpack      struct{} `msgpack:",asArray"`
	UUID          string
	SomeString    string
	AnotherString string
}

const (
	luaProc  = "luaproc"
	cProc    = "libcproc.cproc"
	cppProc  = "libcppproc.cppproc"
	rustProc = "librustproc.rustproc"
)

func main() {
	conn, err := tarantool.Connect("localhost:3301", tarantool.Opts{
		User: "guest",
	})

	if err != nil {
		log.Fatalln(err)
	}

	u, err := uuid.NewRandom()
	if err != nil {
		log.Fatalln(err)
	}

	resp, err := conn.Call17(cppProc, &Args{
		UUID:          u.String(),
		SomeString:    "some string",
		AnotherString: "another string",
	})
	if err != nil {
		log.Fatalln(err)
	}

	log.Printf("%+v\n", resp.Data)
}

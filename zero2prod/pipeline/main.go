package main

import (
	"context"
	"fmt"
	"os"

	"dagger.io/dagger"
)

func main() {
    ctx := context.Background()

    // initialize Dagger client
    client, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stdout))
    if err != nil {
        panic(err)
    }
    defer client.Close()



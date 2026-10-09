package fetch

import (
	"errors"
	"net/http"
)

// ErrTooManyAttempts is returned when every attempt fails.
var ErrTooManyAttempts = errors.New("fetch: too many attempts")

// Fetch sends a GET request to url and returns the response.
func Fetch(client *http.Client, url string) (*http.Response, error) {
	return client.Get(url)
}

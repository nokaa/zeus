package main

import (
	"github.com/boltdb/bolt"
)

func main() {
	db, err := bolt.Open("zeus.db", 0600, nil)
	if err != nil {
		return
	}

	err = db.Update(func(tx *bolt.Tx) error {
		bucket, err := tx.CreateBucketIfNotExists([]byte("bucket"))
		if err != nil {
			return err
		}

		return bucket.Put([]byte("key"), []byte("value"))
	})

	db.Close()
}

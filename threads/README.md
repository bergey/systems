Compare threading models, using a simple model problem.

## model of work to be done

All programs will:
1. listen for HTTP requests
2. parse a **document ID** from the request
3. look up the specified document on disk
4. increment the count in the document
5. write back to disk
6. send an HTTP response with the new count

For now, each document is a DOCSIZE slice of a single file that contains the same byte repeated (the count).  Since every document is the same size, finding a given document is easy.  Alas, this requires serializing all accesses.

Instead, we should shard the documents among a configurable number of files.  I presume it will be necessary to tune the shard count to keep contention low, but staying below file handle limits no matter how many TCP connections or documents we have.  Wait until we can observe contention.

The mutexes needed to avoid lost writes are part of each threading model.  Might experiment with document size, or with a better DB design, like a WAL / Snapshot, or an LSM.

## Threading

Compare several styles:

1. one Linux thread per TCP connection
2. `tokio`, one Rust thread per TCP connection
3. one thread per core, explicit async IO with `io_uring`
4. [ractor](https://github.com/slawlor/ractor)

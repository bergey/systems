Compare threading models, using a simple model problem.

## model of work to be done

All programs will:
1. listen for HTTP requests
2. parse a **document ID** from the request
3. look up the specified document on disk
4. increment the count in the document
5. write back to disk
6. send an HTTP response with the new count

For now, each document is a 1kB sequence in a single file that contains the same byte repeated (the count).  The mutexes needed to avoid lost writes are part of each threading model.  Might experiment with document size, or with a better DB design, like a WAL / Snapshot, or an LSM.

## Threading

Compare several styles:

1. one Linux thread per TCP connection
2. `tokio`, one Rust thread per TCP connection
3. one thread per core, explicit async IO with `io_uring`
4. [ractor](https://github.com/slawlor/ractor)

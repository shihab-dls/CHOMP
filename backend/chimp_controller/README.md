# Chimp Controller

A small shim service which is designed to listen to the `imageCreated` subscription endpoint of the `targeting` service and generate jobs for `chimp_chomp`. When `chimp_chomp` completes the job this service will format the response and send it to the `targeting` service.

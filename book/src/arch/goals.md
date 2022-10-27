# Goals

Xvc is an CLI MLOps tool to track file, data, pipeline, experiment, model versions.

It has the following goals:

- Enable to track any kind of files, including large binary, data and models in Git.
- Enable to get subset of these files.
- Enable to remove files from workspace temporarily, and retrieve them from cache.
- Enable to upload and download these files to/from a central server.
- Enable users to run pipelines composed of commands.
- Be able to invalidate pipelines partially.
- Enable to run a pipeline or arbitrary commands as experiments, and store and retrieve them.

Xvc users are data and machine learning professionals that need to track large amounts of data.
They also want to run arbitrary commands on this data when it changes.
Their goal is to produce better machine learning models and better suited data for their problems.

We have three quality goals:

- **Robustness**: The system should be robust for basic operations.
- **Performance**: The overall system performance must be within the ballpark of usual commands like `b3sum` or `cp`.
- **Availability**: The system must run on all major operating systems.

Xvc users work with large amounts of data.
They want to depend on Xvc for basic operations like tracking file versions, and uploading these to a central location.

They don't want to wait too long for these operations on common hardware.

They would like to download their data to any system running various operating systems.

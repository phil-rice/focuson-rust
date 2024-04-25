# Temporal ++

## Purpose
This is a tool for me to work on while learning rust.

## Motivation
I like the ideas behind the [Temporal](https://temporal.io/) project. I want
to see how I can use the same ideas in a much lighter weight way. I think it is
possible to do a tradeoff between speed and resiliance. While I really like Temporal's
basic core premise, and the the fantastic resiliance the current approach gives you, I find
that I don't need that level of resiliance in my current projects. 

For example when I am indexing a load of data for an elastic search project, if a single batch
dies I can just rerun it. But I still want the benefits of Temporal for things like retry logic

## Approach
The core idea is the event sourcing. I will support a number of event sources. Some in memory, some
on the local file system, some on azure blob storage with append blobs. In this way we can trade off
speed and resiliance.

## Glossary

### Workflow
In Temporal, a **Workflow** represents the orchestration or coordination of activities that are
designed to achieve a specific business goal. It is a series of steps that outline a business 
process, and it can involve decision-making algorithms that react to external events. Workflows
in Temporal are fault-tolerant and can run for a prolonged duration, ensuring the completion of
all steps even in the event of process failures or restarts.

### Activity
An **Activity** represents a single, logical unit of work in a Temporal workflow. Activities 
encapsulate the actual implementation of tasks that the workflow manages. They are the 
functions executed to perform a job within a workflow, like making an HTTP request, processing 
a file, or handling an API call. Activities can be retried, timed out, and scheduled according 
to the workflow's requirements.

### Worker
A **Worker** is a service that executes the workflows and activities defined by the developer.
Workers poll the Temporal server for tasks, execute the tasks, and communicate task completion
or failure back to the server. A Worker can host and run multiple workflows and activities, 
and it's designed to handle intermittent failures gracefully.

### Task Queue
A **Task Queue** is a mechanism within Temporal that routes workflows and activities to Workers 
that are capable of executing them. When a workflow or activity is started, it is assigned to a
task queue, and workers that are listening on that task queue pick it up for execution. This
helps in efficiently distributing work across multiple workers based on their availability and 
capacity.

### Event History
**Event History** in Temporal is a log that records all events that occur during the execution
of a workflow. It is crucial for facilitating Temporal's fault tolerance capabilities. When a
workflow is executed, this history is used to determine the state of the workflow, including
whether specific activities need to be executed or have already been completed. This mechanism
allows workflows to resume from where they left off in case of failures or restarts, without 
losing state or repeating successfully completed activities. In our system we call this the 
Event Store

### Event Store
The **Event Store** is a data store that maintains the event history of a workflow. When serialised
to a file each event is stored as a line in the file. The event store is append only and we'll use
OS file locks or similar to ensure that only one process can write to the file at a time.

I am not sure how abstract our events will be yet. 

### CAS Storage
**CAS Storage** (Content-Addressable Storage) is a type of data storage used in the system to 
anage large parameters and results efficiently. In this approach, the content itself determines
the address where it is stored, using the hash of the content as a unique identifier (ID). 
This mechanism allows for efficient, deduplicated storage, as only one copy of a given piece of 
content is kept, regardless of how many times it is used. The system can utilize various storage
backends such as in-memory data structures, local filesystems, or cloud-based blob storage like 
Azure Blob Storage. This strategy is particularly advantageous for keeping the event store 
streamlined, as it significantly reduces the size of the events by offloading large data objects 
to CAS storage. It also adds significant resiliance to corruption and malevolent hacking.




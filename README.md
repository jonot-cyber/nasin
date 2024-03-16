# nasin
A program to schedule getting tasks done

## Purpose
I tend to struggle to get tasks done, especially if I have multiple
things to do.  Usually, people recommend making some sort of a
schedule, but I don't want to.  Eventually, I started thinking of
operating systems. They typically use algorithms to dynamically
schedule work to be done. I wanted to try that for me (not a
computer), which actually worked pretty well.

I initially implemented this [As a
website](https://github.com/jonot-cyber/tasks), which worked fine, but
I wanted it as a native desktop app. Also, I wanted to make a native
desktop app.

## Scheduler
The scheduler in this is based off of [Fixed-priority pre-emptive
scheduling](https://en.wikipedia.org/wiki/Fixed-priority_pre-emptive_scheduling). This
is a scheduling algorithm meant for real-time systems, which are
systems that have deadlines and priorities for different tasks. This
makes sense for a human person, where the same two things apply.

The way the algorithm works is at each stage, it selects the task with
the highest priority (lowest number) to do first. If there are
multiple tasks with this priority, it will cycle between them
all. This approach is simple, but has a problem. For a computer, tasks
with low priority will be "starved" by tasks with a high priority, and
never get to run. Human people can have a similar problem where they
get bored of doing the same thing over and over again. To fix both of
these problems, we add something called "aging." At each stage, we add
one to the age of each task, which represents how long a task has gone
with nothing being done to it. This is reset after a task is worked
on. After this is set, the oldest task has its priority temporarily
increased (and its age reset). After a task is run, its priority will
be reset to what it originally was. This way, tasks with a lower
priority can be run. This is good in some cases, for example, a
project that has a far-off due date, but should still be worked on
occasionally. Another example is giving yourself brakes every so
often.

Warning: This does not alleviate the problem of simply not wanting to
do a task. This is something that I doubt can be fixed with shoddy
code anyways, so it will be considered out of scope.

## Running
The project can be run like any normal rust program with `cargo run` or `cargo run --release`. If you want to build a Flatpak, you can use the following command:
```sh
flatpak-builder --user --install --force-clean build-dir me.jonot.Nasin.json
```
This app is not on Flathub because it is not very good.

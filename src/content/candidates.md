Long have I pondered the best question to gauge a candidate's level of maturity in software engineering. Reliably dodging "interview prep" can be tricky. Asking about specific skills encourages deceit. I've decided the question has to be open-ended, promise a "happy path" forward, but leave traps along that path that experienced engineers will highlight, caution against, and avoid. This is what I've got so far.

**You're tasked with upgrading the Python "requests" library to support Python 4.
This library provides an HTTP client, and it's a dependency for millions of Python projects.
You've just installed Python 4.0 and imported the library. It loads fine, but it crashes the first time you try to use it.
The error message reads, _"InvalidSchema: No connection adapters were found."_ What do you do next?**

Just ask the question, then let the candidate talk.

## Expected from Senior/Staff Engineer

**Run my test suite / improve test coverage / confirm in Python 3**

This gets to the crux of the problem: the candidate had no control. Confirm this behavior is actually a regression before debugging further.

Great candidates will refuse to go any further without verifying test coverage. Fixing this error, in a library this extensively used, is almost certain to cause regressions for downstream users.

Top candidates will ignore the above, declare the public interfaces sacred, rip out the internals, and specify an appropriate version bump.

**Check the release notes for Python 4**

This gets second billing because people who say this know what's up. It's a great place to start in general, and it's never the wrong answer.

Great candidates will find the Python 4 SME, even if they're in Finland, and schedule a time to chat with them.

Top candidates will ask why the developer of the 'requests' library wasn't directly involved in the launch of Python 4 from the start. They would have installed the first release candidate a year ago.

**Search source code or repository for "InvalidSchema" / "No connection adapters were found."**

This is the most logical diving-in step. It can help the candidate understand the scope of the problem; will it be a one-line change or a massive refactor?

Great candidates will search the entire organization's repository, not just the source. Problems are rarely isolated.

Top candidates will note that hearing words like "schema," "connection," and "http" together suggests an error at the protocol level.

## Expected from Junior/Mid-Level Engineer

**Search Google for the error message**

Nothing wrong with research. Google's a great default first step.

Great candidates will limit their search to specific sites that cater to low-level developers.

Top candidates will intuit that the task has probably never been attempted before and doubt the efficacy of a search.

**Start a debug terminal / try to trace the code that's failing**

It's good troubleshooting, but it's a little too early to be looking at traces.

Great candidates will talk about debugging intricacies, likke setting breakpoints and trying to jump the failing instruction.

Top candidates will only start here because "the debugger is always running when I'm developing, so why not?"

## Expected from Intern/Junior Engineer

**Ask someone with more experience for help**

Candidate literally hasn't tried anything on their own yet.

Great candidates will post their question in a common area, like a Slack channel, rather than "asking someone"

Top candidates will ask people outside the company, like a developer community, for help.

# 2023-01-13
First rust application I done. I like clean architechture so tried to make
own modules for domain, application & infra parts. As app works in console
presentation and application are pretty same. In study repo I first even
made separare cargo modules. But quickly learned that separation of modules
kinda is almost same thing and simple stuff theres no need for more
separation. Separation of module tests and integration tests seemed so
nice too.

Did not know how to reserve memory for the todo list state so ended up
making silly immutable state machine, two times, that work on top each
other. Entry point just calls application loop that is state machine
running domain state machine.

Fun and funny to kinda brute force something out of code when you only have
very limited knowledge of new language. But the ability to browser and read
source code that contains documentation in itself makes experience great.

Plan still is to add the infra module that can store the state into
a file on users home folder.

Maybe learn better ways to manage state too before rushing into new app
which is already tickling in the mind.

Title: Problem Set 1
Author: Nick Skelsey

problem 1:
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:10.0.12) Gecko/20100101 Firefox/10.0.12 Iceweasel/10.0.12

	It reports Mozilla, because I am running a version of firefox, the (X11 ... ) refers to the window manager that I am running and X86 is the architecture. The UA reports gecko and a build version describing the DOM renderer used by my version of firefox. The last bit about Iceweasel is there because I am running debian and the default install backports a secure version of firefox to avoid any copyright issues that come with non-free images and such.


problem 2:
     Rust considers global mutable variables unsafe because there is nothing stopping another completely unrelated thread of the program from editing the value of that variable otherwise. This would mean that the value could change at any point in a multithreaded context.

problem 3:
     solved in code.	
	
problem 4:
	solved in code.

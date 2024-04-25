def hello := "world"

def myOtherFunNoArg : String := "Boo"

@[export myFunNoArg]
def myFunNoArg : String :=
  "Lean says hi :)"

#check myFunNoArg

@[export myFunOneArg]
def myFunOneArg (s: String) : String :=
  String.append "You said: " s

package xchemlab.compound_library

import data.xchemlab
import rego.v1

default read_compound = {"allowed" : false}
default write_compound = {"allowed" : false}

read_compound = response if {
    xchemlab.valid_token
    response := {
        "allowed": true,
        "subject": xchemlab.subject,
    }
}

write_compound = response if {
    xchemlab.valid_token
    response := {
        "allowed" : true, 
        "subject" : xchemlab.subject,
    }
}

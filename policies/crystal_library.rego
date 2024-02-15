package xchemlab.crystal_library

import data.xchemlab
import rego.v1

default read_crystal = {"allowed" : false}
default write_crystal = {"allowed" : false}

read_crystal = response if {
    xchemlab.valid_token
    response := {
        "allowed": true,
        "subject": xchemlab.subject,
    }
}

write_crystal = response if {
    xchemlab.valid_token
    response := {
        "allowed" : true, 
        "subject" : xchemlab.subject,
    }
}

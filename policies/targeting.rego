package xchemlab.targeting

import data.xchemlab
import future.keywords.if

default read_image = {"allowed": false}

default write_image = {"allowed": false}

default read_prediction = {"allowed": false}

default write_prediction = {"allowed": false}

read_image = response if {
    xchemlab.valid_token
    response := {
        "allowed": true,
        "subject": xchemlab.subject
    }
}

write_image = response if {
    xchemlab.valid_token
    response := {
        "allowed": true,
        "subject": xchemlab.subject
    }
}

read_prediction = response if {
    xchemlab.valid_token
    response := {
        "allowed": true,
        "subject": xchemlab.subject
    }
}

write_prediction = response if {
    xchemlab.valid_token
    response := {
        "allowed": true,
        "subject": xchemlab.subject
    }
}

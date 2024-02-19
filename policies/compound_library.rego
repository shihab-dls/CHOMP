package xchemlab.compound_library

import data.xchemlab

default read_compound = {"allowed" : false}
default write_compound = {"allowed" : false}

read_compound = {"allowed": true, "subject": xchemlab.subject} {
    xchemlab.valid_token
}

write_compound = {"allowed" : true, "subject" : xchemlab.subject} {
    xchemlab.valid_token
}

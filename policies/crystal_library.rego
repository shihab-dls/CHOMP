package xchemlab.crystal_library

import data.xchemlab

default read_crystal = {"allowed" : false}
default write_crystal = {"allowed" : false}

read_crystal = {"allowed": true, "subject": xchemlab.subject} {
    xchemlab.valid_token
}

write_crystal = {"allowed" : true, "subject" : xchemlab.subject} {
    xchemlab.valid_token
}

package xchemlab.compound_soaking

import data.xchemlab

default read_soaked_compound = {"allowed" : false}
default write_soaked_compound = {"allowed" : false}

read_soaked_compound = {"allowed": true, "subject": xchemlab.subject} {
    xchemlab.valid_token
}

write_soaked_compound = {"allowed" : true, "subject" : xchemlab.subject} {
    xchemlab.valid_token
}

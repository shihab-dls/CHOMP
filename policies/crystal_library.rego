package xchemlab.crystal_library

import data.xchemlab

default read_crystal_wells = {"allowed" : false}
default write_crystal_wells = {"allowed" : false}
default read_crystal_plates = {"allowed" : false}
default write_crystal_plates = {"allowed" : false}

read_crystal_wells = {"allowed": true, "subject": xchemlab.subject} {
    xchemlab.valid_token
}

write_crystal_wells = {"allowed" : true, "subject" : xchemlab.subject} {
    xchemlab.valid_token
}

read_crystal_plates = {"allowed": true, "subject": xchemlab.subject} {
    xchemlab.valid_token
}

write_crystal_plates = {"allowed" : true, "subject" : xchemlab.subject} {
    xchemlab.valid_token
}

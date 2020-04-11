# -*- coding: utf8 -*-

import os
import json
def exec(cmd):
    a=os.popen(cmd)
    t= "".join([x for x in a.readlines()])
    return t

def main():
    #cmd="target/debug/qqmusic 从开始到现在"
    #cmd="target/release/qqmusic 从开始到现在"
    cmd="./qqmusic 从开始到现在"
    r=exec(cmd)
    #r1=json.loads(r)
    #print(r1)
    return r

def main_handler(event, context):
    r=main()
    print(r)
    return r

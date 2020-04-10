package main

import (
	"fmt"
	"bytes"
	"os/exec"
	"io"
	"bufio"
	"strings"
)

func exec_shell(s string) (string, error){
    cmd := exec.Command("/bin/bash", "-c", s)
    var out bytes.Buffer
    cmd.Stdout = &out
    err := cmd.Run()
    //checkErr(err)
    return out.String(), err
}


func exec1(c[]string) (bool,string ){
	s:=""
	cmd := exec.Command(c[0],c[1:]...)
    stdout, err := cmd.StdoutPipe()
    if err != nil {
        fmt.Println(err)
        return false,s
    }
    cmd.Start()
    reader := bufio.NewReader(stdout)
    for {
        line, err2 := reader.ReadString('\n')
        if err2 != nil || io.EOF == err2 {
            break
        }
		s+=line
    }
    cmd.Wait()
    return true,s
}

func test1() {
	cmd:="target/debug/qqmusic 从开始到现在"
	a,_:=exec_shell(cmd)
	fmt.Println(a)
}

func test2(){
	cmd:="target/debug/qqmusic 从开始到现在"
	cmd1:=strings.Split(cmd," ")
	//cmd1:=[]string{"target/debug/qqmusic","从开始到现在"}
	_,s:=exec1(cmd1)
	fmt.Println(s)
}

func main(){
	test2()
}

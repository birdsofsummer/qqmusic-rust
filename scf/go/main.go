package main

import (
	"fmt"
	"bytes"
	"os/exec"
	"io"
	"bufio"
	"strings"
    "context"
    "github.com/tencentyun/scf-go-lib/cloudfunction"
	. "github.com/tencentyun/scf-go-lib/cloudevents/scf"
	"encoding/json"
)

var CORS = map[string]string{
        "access-control-allow-origin": "*",
        "access-control-allow-methods": "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
        "access-control-allow-headers": "accept,accept-encoding,cf-connecting-ip,cf-ipcountry,cf-ray,cf-visitor,connection,content-length,content-type,host,user-agent,x-forwarded-proto,x-real-ip,accept-charset,accept-language,accept-datetime,authorization,cache-control,date,if-match,if-modified-since,if-none-match,if-range,if-unmodified-since,max-forwards,pragma,range,te,upgrade,upgrade-insecure-requests,x-requested-with,chrome-proxy,purpose,accept,accept-language,content-language,content-type,dpr,downlink,save-data,viewport-width,width",
        "access-control-max-age": "1728000",
}    


func Format_res(body string,err error) (r APIGatewayProxyResponse) {
	if err!=nil {
		return APIGatewayProxyResponse{
				IsBase64Encoded:false,
				StatusCode:500,
				Headers:CORS,
				Body:"",
			}
	}
	return APIGatewayProxyResponse{
		IsBase64Encoded:false,
		StatusCode:200,
		Headers:CORS,
		Body:body,
	}
}


func Echo (ctx context.Context, event APIGatewayProxyRequest) (string,error) {
	s,_:=json.Marshal(event)
	body:=string(s)
	return body,nil
}

func QQmusic(ctx context.Context, event APIGatewayProxyRequest) (string,error) {
	//event.Body;
	//event.QueryString

	//s,_:=json.Marshal(event)
	//body:=string(s)
	//return body,nil
	cmd:="/var/user/qqmusic 从开始到现在"
	//cmd1:=strings.Split(cmd," ")
	//cmd1:=[]string{"target/debug/qqmusic","从开始到现在"}
	_,l:=exec1([]string{"ls","."})
    _,p:=exec1([]string{"pwd"})
	fmt.Println(l)
	fmt.Println(p)
	s,_:=exec_shell(cmd)
	//ok,s:=exec1(cmd1)
	fmt.Println(s)
	return s,nil
}

func create_router() (map[string]func(context.Context,APIGatewayProxyRequest) (string,error)){
	r:=make(map[string]func(context.Context,APIGatewayProxyRequest) (string,error))
	r["/"]=Echo
	r["/music"]=QQmusic
	return r
}




func App (ctx context.Context, event APIGatewayProxyRequest) (APIGatewayProxyResponse,error) {
	fmt.Println(event)
	p1:=event.RequestContext.Path
	p2:=event.Path
    p:=strings.Replace(p2, p1, "", -1 ) 
	r:=create_router() 
    f, ok := r[p] 
    if (!ok) {
		f=Echo
    }
	return Format_res(f(ctx,event)),nil
}


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
	cmd:="./qqmusic 从开始到现在"
	cmd1:=strings.Split(cmd," ")
	//cmd1:=[]string{"target/debug/qqmusic","从开始到现在"}
	_,s:=exec1(cmd1)
	fmt.Println(s)
	
}

func main(){
	//test2()
    cloudfunction.Start(App)
}

package main
import (
    "context"
	"strings"
  //  "fmt"
  //	"encoding/json"
	. "github.com/tencentyun/scf-go-lib/cloudevents/scf"
   // "github.com/tencentyun/scf-go-lib/cloudfunction"
)
//type F func(string) string
type F func(context.Context,APIGatewayProxyRequest) (string,error)
type Fs  []F
type Fhash map[string]F
type Router struct {
	r Fhash
	prefix string
	middleware Fs
}

func encode(a string,b string)(string){
	return a+"_"+b
}

func decode(s string)(a string,b string){
	r:=strings.Split(s, ",")
	a=r[0]
	b=r[1]
	return a,b
}


func (r Router) GET (path string,f F){
	r.r[encode("GET",path)]=f
}
func (r Router) POST (path string,f F){
	r.r[encode("POST",path)]=f
}
func (r Router) PUT (path string,f F){
	r.r[encode("PUT",path)]=f
}
func (r Router) DELETE (path string,f F){
	r.r[encode("DELETE",path)]=f
}
func (r *Router) Prefix (p string){
	r.prefix=p
}
func (r *Router) Use (f F){
	m:=r.middleware
	r.middleware=append(m,f)
}
func (r Router) Find (method string,path string)( F,bool) {
	p1:=r.prefix
	p2:=path
	if (len(p1)!=0){
		p2=strings.Replace(path,p1,"",1)
	}
	p:=encode(method,p2)
	f,ok:=r.r[p]
	return f,ok
}


func (r Router) Run (ctx context.Context, event APIGatewayProxyRequest) ( APIGatewayProxyResponse){
	method:= event.HTTPMethod
	p1:=event.RequestContext.Path
	p2:=event.Path
    path:=strings.Replace(p2, p1, "", -1 ) 
	f,ok:=r.Find(method,path)
	//mid:=r.middleware
	if (ok){
		return Format_res(f(ctx,event))
	}else{
		return Format_res(Echo(ctx,event))
	}
}


func Create_Router(){
    var r Router
	r.r=make(Fhash)
	//r.Prefix("/api/")
	r.GET("/",Echo)
	r.GET("/music",QQmusic)
}








//--------------------------------test------------------------------------------------

/*


func reduce1(a string,g Fs) string{
	if len(g) == 0{
		return a
	}else {
		return reduce(g[0](a),g[1:])
	}
}

func (r Router) Run1 (method string,path string,x string) string{
	f,ok:=r.Find(method,path)
	mid:=r.middleware
	if (ok){
		s:=reduce1(x,mid)
		return f(s)
	}else{
		return "403"
	}
}


type F func(string) string

func add(x string) string{return "<<<<"+x+">>>>"}
func add1(x string) string{
	return "???"+x
}
func add2(x string) string{
	return "^^^"+x
}
func test_router() Router{
    var r Router
	r.r=make(Fhash)
	r.Prefix("/api/")
	r.GET("ccc",add)
	r.Use(add1)
	r.Use(add2)
    return r
}

func main(){
	r:=test_router()
	z:=r.Run1("GET","/api/ccc","yyy")
    fmt.Println(z)
}
*/

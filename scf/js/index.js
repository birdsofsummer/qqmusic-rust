const child_process=require("mz/child_process")
const exec = child_process.exec

init=async ()=>{
   let cmd="target/debug/qqmusic 从开始到现在"
   let t=await exec(cmd)
   let d=JSON.parse(t[0])
   console.log(d)
}

init()

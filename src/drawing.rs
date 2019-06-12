extern crate rand;

use drawing::rand::Rng;
use program::{WiretagMarker,Wireassign,FakeMux,RegisterBank};

#[derive(Debug,Clone)]
pub struct Drawingtool {
    pc_output:String,
    pc_reg:String,
    i10_assignments:Vec<WiretagMarker>,
    wires: Vec<Wireassign>,
    muxes:Vec<FakeMux>,
    registers:Vec<RegisterBank>,
    pipeline_regs:Vec<String>,
}
impl Drawingtool {

    pub fn new(components:(String,String,Vec<WiretagMarker>, Vec<Wireassign>,Vec<FakeMux>, Vec<RegisterBank>)) -> Drawingtool{
        //let components=r_p.parse_file();
        Drawingtool{pc_output:components.0,pc_reg:components.1,i10_assignments:components.2,wires:components.3,muxes:components.4,registers:components.5,pipeline_regs:Vec::new()}
        
    }
    pub fn set_pipeline_regs(& mut self, regs:Vec<String>){
        self.pipeline_regs=regs;
    }
    pub fn start(&self, pipeline:bool) {
        if !pipeline
        {
            println!("digraph g {{");
            println!("graph [fontsize=30 labelloc=\"t\" label=\"\" splines=true overlap=false rankdir = \"LR\"];");
            println!("ratio = auto;");
            self.output_special_regs();
            self.print_reg_and_assignments();
            self.print_muxes();
            println!("}}");
            
        }
        else 
        {
            println!("digraph g {{");
            println!("graph [fontsize=30 labelloc=\"t\" label=\"\" splines=true overlap=false rankdir = \"LR\"];");
            println!("ratio = auto;");
            self.output_special_regs();
            self.print_reg_and_assignments();
            self.print_muxes();
            println!("}}"); 
            println!("{:#?}",self.pipeline_regs);
        }
        return

    }
    /*fn output_reg_var(&self,name: String,size:String,default:String) -> RetType {
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; {name} -&gt; {size}={default} </td></tr>");
    }*/
    fn output_special_regs(&self) {
        println!("  \"pc\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">Pc</font></td></tr></table>> ];");
        println!("  \"Stat\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">Stat</font></td></tr></table>> ];");
        print!("  \"reg_read\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">reg_read</font></td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_srcA -&gt; 4=REG_NONE </td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_srcB -&gt; 4=REG_NONE </td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_outputA -&gt; 64=0 </td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_outputB -&gt; 64=0 </td></tr>");
        println!("</table>> ];");
        print!("  \"reg_write\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">reg_write</font></td></tr>");
		print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_dstE -&gt; 4=REG_NONE </td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_inputE -&gt; 64=0 </td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_dstM -&gt; 4=REG_NONE </td></tr>");
        print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; reg_inputM -&gt; 64=0 </td></tr>");
        println!("</table>> ];");
        print!("  \"mem\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">mem</font></td></tr>");
        println!("</table>> ];");
    }
    fn check_input(&self,val:String) -> (String,String,bool) {
        let mut ret:(String,String,bool)=self.check_if_reg(val.clone());
        if ret.2==false {
            ret=self.check_if_reg_input(val.clone());
        }
        if ret.2==false {
            ret=self.check_if_mem_input(val.clone());
        }
        return ret;
    }
    fn check_if_reg_input(&self,val:String) -> (String,String,bool) {
        if val==String::from("reg_srcA") {
            return (String::from("reg_read"),val,true);
        }
        if val==String::from("reg_srcB") {
            return (String::from("reg_read"),val,true);
        }
        if val==String::from("reg_dstE") {
            return (String::from("reg_write"),val,true);
        }
        if val==String::from("reg_inputE") {
            return (String::from("reg_write"),val,true);
        }
        if val==String::from("reg_dstM") {
            return (String::from("reg_write"),val,true);
        }
        if val==String::from("reg_inputM") {
            return (String::from("reg_write"),val,true);
        }
        if val==String::from("reg_outputA") {
            return (String::from("reg_read"),val,true);
        }
        if val==String::from("reg_outputB") {
            return (String::from("reg_read"),val,true);
        }
       // println!("false for reg");
        return (val.clone(),val.clone(),false);
    }
    fn check_if_mem_input(&self,val:String) -> (String,String,bool) {
        if val==String::from("mem_readbit") {
            return (String::from("mem"),val,true);
        }
        if val==String::from("mem_addr") {
            return (String::from("mem"),val,true);
        }
        if val==String::from("mem_writebit") {
            return (String::from("mem"),val,true);
        }
        if val==String::from("reg_inputE") {
            return (String::from("mem"),val,true);
        }
        if val==String::from("mem_input") {
            return (String::from("mem"),val,true);
        }
        if val==String::from("mem_output") {
            return (String::from("mem"),val,true);
        }
        //print!("false for mem");
        return (val.clone(),val.clone(),false);

    }
    fn check_if_reg(&self,val:String) -> (String,String,bool) {
        for reg in self.registers.clone(){
            for sig in reg.get_signal(){
                if sig.0==val || sig.1==val{
                    return (reg.get_label(),val,true);
                }
            }
        }
        return (val.clone(),val.clone(),false);
    }

    fn print_label(&self,label1:String,label2:String,label_flag1:bool,label_flag2:bool)  {
        let mut combined_str= String::new();
        if label_flag1
        {
            combined_str.push_str(label1.as_str());
            if label_flag2
            {
                combined_str.push_str("/");
                combined_str.push_str(label2.as_str()); 
            }
        }
        else 
        {
            if label_flag2
            {

                combined_str.push_str(label2.as_str()); 
            }
        }

        print!("[ penwidth = 1 fontsize = 10 fontcolor = \"black\" label = \"{}\" ];",combined_str );
    }
    fn check_if_expr(&self,value:String)-> (String, String, String, bool)  {
        let mut left= String::new();
        let mut left2= String::new();
        let mut middle= String::new();
        let mut right= String::new();
        let mut flag=false;
        let mut skip=false;
        let mut found_second=false;
        let char_vec:Vec<char> = value.chars().collect();
        for (i, c) in value.chars().enumerate() {
            let mut temp=c.to_string();
            if flag==false
            {
                left2.push_str(temp.clone().as_str());
                if self.check_char_for_symbol(temp.clone()).1== true
                {

                    if char_vec[i]==char_vec[i+1]
                    {
                        temp.push_str(char_vec[i+1].to_string().as_str());
                        skip=true;
                        
                    }
                    else {


                    }
                    middle=temp.clone();
                    right=String::new();
                    flag=true;
                    

                }
                else 
                {
                    left.push_str(temp.clone().as_str());

                }
            }

            else 
            {


                if skip==false && !found_second
                {
  

                    if self.check_char_for_symbol(temp.clone()).1== true
                    {
                        if char_vec[i]==char_vec[i+1]
                        {
                        temp.push_str(char_vec[i+1].to_string().as_str());
                        skip=true;
                        }

                        middle=temp.clone();
                        left=left2.clone();
                        right=String::new();
                        found_second=true;
                    }
                    else
                    {
                        
                    
                    right.push_str(temp.clone().as_str());
                    left2.push_str(temp.clone().as_str());
                    }
                }
                else if skip==false && found_second {
                    right.push_str(temp.clone().as_str());
                }
                else if skip==true && !found_second{

                    
                    skip=false;

                }
            }

        }
        if flag==true || found_second
        {
            
               return (left,middle,right,true); 

        }

        else
        {

                return (left,middle,right,false);

        }
    }
    fn count_vec_length(&self, vec_to_count:Vec<String>) -> u8 {
        let mut count:u8=0;
        for _mux in vec_to_count{
            count=count+1;
        }
        return count;
    }
    fn print_box_mux(&self, mux:FakeMux)-> (String,Vec<String>){
        let mut rng = rand::thread_rng();
        let n1: u16 = rng.gen();
        print!("\"{}\" [shape=none, margin=0, label=<",n1.to_string().clone() );
        print!("<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\" CELLPADDING=\"4\">");

        let mut rights=Vec::new();

        let mut used_vals=Vec::new();
        for val in mux.values.clone()
        {

            if !(used_vals.iter().any(|v| v == &val.clone()))
            {
                print!("<TR>");
                let exp= self.check_if_expr(val.clone());
                if !(rights.iter().any(|v| v == &exp.0.clone()))
                {
                    if !exp.0.clone().parse::<i128>().is_ok()
                    {
                        rights.push(exp.0.clone());
                    } 
                }   
                if !(rights.iter().any(|v| v == &exp.2.clone()))
                {
                    if !exp.2.clone().parse::<i128>().is_ok()
                    {
                        rights.push(exp.2.clone());
                    }
                }
                if exp.3==true
                {
                    if self.certain_symbol_to_words(exp.1.clone()) !=String::from("null")
                    {

                            let mut temp_comb= String::new();
                            temp_comb.push_str(exp.0.as_str());
                            temp_comb.push_str(self.certain_symbol_to_words(exp.1.clone()).as_str());
                            temp_comb.push_str(exp.2.as_str());
                            print!("<TD>\"{}\"</TD>",temp_comb.clone());
                            
                            

                    }
                    else 
                    {
                            println!("<TD>\"{}\"</TD>",val.clone());
                        
                    }
                }
                else 
                {
                    if exp.0.clone().parse::<i128>().is_ok() && val.clone().parse::<i128>().unwrap()>100
                    {
                        println!("<TD>\"{}\"</TD>",val.parse::<i128>().unwrap());
                    }    
                    else 
                    {
                        println!("<TD>\"{}\"</TD>",val.clone());
                    }

                }
                print!("</TR>");
                used_vals.push(val.clone());
            }
        }
        

        println!("</TABLE>>];");
        return (n1.to_string().clone(),rights);
    }
    fn print_muxes(&self) {

        let mut printed_reg_outs= Vec::new();
        for mux in self.muxes.clone()
        {
            if self.count_vec_length(mux.values.clone())>0 
            {
                let right=self.check_input(mux.out.clone());
                let results=self.print_box_mux(mux.clone());
                for left in results.1
                {
                    let text=self.check_input(left.clone());
                    if text.2==false
                    {
                        self.print_line(text.0,results.0.clone(),text.1.clone(),results.0.clone(),text.2,false);
                    }
                    else 
                    {
                        if !(printed_reg_outs.iter().any(|v| v == &left))
                        {
                        self.print_basic_node(left.clone());
                        printed_reg_outs.push(left.clone());
                        self.print_line(text.0.clone(),text.1.clone(),text.1.clone(),text.1.clone(),false,false);
                        }
                        self.print_line(text.1.clone(),results.0.clone(),text.1.clone(),text.1.clone(),text.2,text.2);                                  
                    }
                        
                }
                    if right.2==true
                    {
                        if !(printed_reg_outs.iter().any(|v| v == &right.1))
                        {

                            self.print_basic_node(right.1.clone());
                            printed_reg_outs.push(right.1.clone());
                            self.print_line(right.1.clone(),right.0.clone(),right.0.clone(),right.1.clone(),false,false);
                        }            
                    }
                        print!("\"{}\"",results.0.clone() );
                        print!(" -> ");
                        print!("\"{}\"",right.1.clone() ); 
                        println!("");
                }
        }
    }
    fn print_reg_and_assignments(&self){

        self.print_reg_gv();

        for val in self.wires.clone(){

                let mut w_a=val.clone();
                let mut temp_str= String::new();
                if w_a.right.len()==1 
                {
                    let left=self.check_input(w_a.right[0].clone());
                    let right=self.check_input(w_a.left.clone());
                    self.print_line(left.0.clone(),right.0.clone(),left.1.clone(),right.1.clone(),left.2.clone(),right.2.clone());
                }
                if w_a.right.len()==2 
                {
                  temp_str.push_str(self.check_input(w_a.right[0].clone()).0.as_str());
                    println!("{:?}",temp_str );
                    temp_str.push_str(self.check_input(w_a.right[1].clone()).0.as_str());
                    println!("{:?}",temp_str );
                }
                if w_a.right.len()==3 
                {
                    let mut middle= String::new();
                    let mut left=self.check_input(w_a.right[0].clone());
                    temp_str.push_str(left.0.as_str());
                    middle.push_str(left.1.as_str());
                    middle.push_str(w_a.right[1].clone().as_str());
                    if !(self.check_if_num(w_a.right[2].clone())) 
                    {
                        middle.push_str(w_a.right[2].clone().as_str());
                    }
                    else
                    {
                        if w_a.right[2].clone().parse::<i128>().unwrap()>100
                        {
                            middle.push_str(format!("{:X}", w_a.right[2].clone().parse::<i128>().unwrap()).as_str());
                        }
                        else 
                        {
                            middle.push_str(w_a.right[2].clone().as_str());
                        }

                    }
                    if !(self.check_if_num(temp_str.clone())) 
                    {
                        self.print_line(temp_str,middle.clone(),left.1.clone(),left.1.clone(),left.2.clone(),false);
                    }
                    temp_str= String::new();
                    left=self.check_input(w_a.right[2].clone());
                    temp_str.push_str(left.0.as_str());
                    if !(self.check_if_num(temp_str.clone())) 
                    {
                        self.print_line(temp_str,middle.clone(),left.1.clone(),left.1.clone(),left.2.clone(),false);
                    }
                    left=self.check_input(w_a.left.clone());
                    self.print_line(middle.clone(),left.0.clone(),left.1.clone(),left.1.clone(),false,false);


                }

            

        }
        
    }
    fn print_basic_node(&self, name:String){
            print!("\"{}\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">{}</font></td></tr>",name.clone(),name.clone() );
            print!("</table>>];");
            println!("");

    }
    fn print_reg_gv(&self){
   
        for reg in self.registers.clone(){
            print!("\"{}\" [ style = \"filled, bold\" penwidth = 5 fillcolor = \"white\" fontname = \"Courier New\" shape = \"Mrecord\" label =<<table border=\"0\" cellborder=\"0\" cellpadding=\"3\" bgcolor=\"white\"><tr><td bgcolor=\"black\" align=\"center\" colspan=\"2\"><font color=\"white\">{}</font></td></tr>",reg.get_label(),reg.get_label() );
            for sig in reg.get_signal().clone()
            {
                print!("<tr><td align=\"left\" port=\"r0\">&#40;0&#41; {} -&gt; 64=0 </td></tr>",sig.0);
            }
            print!("</table>>];");
            println!("");

        }
    }

fn check_char_for_symbol(&self,sym: String) -> (String,bool) {
    if self.symbol_to_words(sym.clone()) != String::from("null"){
        return (sym.clone(),true);
    }
    else {
        return (String::from("null"),false);
    }

}
pub fn certain_symbol_to_words(&self,sym:String) -> String
{

                                        

        if sym== String::from("&") 
           { return String::from("_and_");}

        else if sym== String::from("&&")
          {  return String::from("_andand_");  }

        else {
            return String::from("null");
        }

                                   

}
pub fn symbol_to_words(&self,sym:String) -> String
{

                                        
        if sym==String::from("+")
            {return String::from("addition");}
        else if sym== String::from("-") 
            {return String::from("subtraction");}
        else if sym== String::from("*") 
           { return String::from("multiplication");}
        else if sym==String::from("/")
            {return String::from("division"); } 
        else if sym==String::from("|")
          {  return String::from("bit or");}
        else if sym== String::from("^")
            {return String::from("xor");}
        else if sym== String::from("&") 
           { return String::from("bit and");}
        else if sym== String::from("=")
            {return String::from("equals");}
        else if sym==String::from("!=")
          {  return String::from("not equal");}
        else if sym== String::from("<=") 
          {  return String::from("less than or equal");}
        else if sym== String::from(">=")
            {return String::from("greater than or equak");}
        else if sym==String::from("<")
           { return String::from("less than");}
        else if sym== String::from(">")
           { return String::from("greater than");}
        else if sym== String::from("&&")
          {  return String::from("logical and");  }
        else if sym== String::from("||")
           { return String::from("logical or");}
        else if sym== String::from(">>")
          {  return String::from("right bit shift");}
        else if sym== String::from("<<")
           { return String::from("left bit shift");}
        else {
            return String::from("null");
        }

                                   

}
pub fn un_op_to_string(&self,sym:String) -> String
{
    if sym==String::from("+")
            {
                return String::from("plus");
            }
        else if sym== String::from("-") 
            {
                return String::from("negate");
            }
        else if sym== String::from("~") 
            { 
                return String::from("Complement");
            }
        else if sym==String::from("!")
            {
                return String::from("Not"); 
            }
        else 
        {
           return String::from("error");                              
        }                             

}
fn check_if_num(&self, string_to_check:String) -> bool {
    if string_to_check.parse::<i128>().is_ok()
    {
        return true;
    }
    else
    {
        return false;
    }
}

fn print_line(&self, left:String, right:String,label1:String,label2:String,label_flag1:bool,label_flag2:bool){
    if left != String::new()
    {
        if !(self.check_if_num(left.clone()))
        {

            print!("\"{}\"",left );
            print!(" -> ");
            print!("\"{}\"",right );
            if label_flag1==true || label_flag2==true
            {
                self.print_label(label1,label2,label_flag1,label_flag2);
            } 
            println!("");
        }
        else 
        {

            let mut rng = rand::thread_rng();
            let n1: u8 = rng.gen();
            print!("\"{}\"",n1.to_string().clone() );
            if left.parse::<i128>().unwrap()>100
            {
                println!("[label=\"{:X}\"];",left.parse::<i128>().unwrap());
            }
            else 
            {
                println!("[label=\"{}\"];",left.clone());
            }
            print!("\"{}\"", n1.to_string().clone());
            print!(" -> ");
            print!("\"{}\"",right );
            if label_flag1==true || label_flag2==true
            {
                self.print_label(label1,label2,label_flag1,label_flag2);
            } 
            println!("");


        }
    }
}



    

}
impl Solution {
    pub fn is_valid(s: String) -> bool {
          let mut v:Vec<char>=vec![];
        for c in s.chars(){
            match c {
                '(' | '[' | '{' => v.push(c),
                _ =>{
                    if let Some(c2)=v.pop(){
                        if !(c2=='('&&c==')') && !(c2=='['&&c==']') && !(c2=='{'&&c=='}'){
                            return false;
                        }
                    }else{
                        return false;
                    }
                }
            }
        }
        v.is_empty()
    }
}
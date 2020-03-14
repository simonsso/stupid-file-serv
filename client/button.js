'use strict';

const e = React.createElement;



class LikeButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = { liked: false , s:props.xitem };
  }
    
  render() {
    if (this.state.liked) { 

      return 'Table'+(tab)+' Orderd.'+this.state.s.itemname+" will be ready at "+((Date.now() / 1000 | 0)+this.state.s.time);
    }
    
    return e(
      'button',
      { onClick: () => {
              let tablenumer=document.getElementById("UITabNum");
              let tab=tablenumer?tablenumer.value:0;

              let qty =1;
              let ans=post_order({
                    table:tab,
                    itemname:this.state.s.itemname,
                    qty:qty,
                    eta:(Date.now() / 1000 | 0)+this.state.s.time,
              });
             }
      },
      this.state.s?this.state.s.itemname:'Like'
    );
  } 
}   



class DeleteButton extends React.Component {
  constructor(props) {
    super(props);
    this.state = { liked: false , xurl:props.xurl };
  }
    
  render() {
    if (this.state.liked) { 
      return " revokation pending"
    }
    return e(
      'button',
      { onClick: () => {
            fetch(this.state.xurl, {
                  method: 'DELETE',
                  headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                  },
                }).then(response => { 
                  if(! response.ok){
                      alert("While removing file. Error:"+response.status+" "+response.statusText);
                  }
                  request_full_tab();
                });
                this.setState({liked:true});
      },},"remove ",);
      
    }
}



var request_full_tab = function(){
        let bartab=fetch('http://localhost:5000/files', {
        method:"GET",
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
        }}
      ).then(response => {
        if ( response.ok  ) {
            response.json().then( jsonobj => {
                              print_full_tab(jsonobj['files']) ;
                            }
            );
        }else{
          window.alert("Error loading file list - reload page manually to retry");
          res = response;
        }
        
        });
}

var print_full_tab=function(resp){
    let domContainer = document.querySelector('#bartab');
    while (domContainer.firstChild) {
      domContainer.removeChild(domContainer.firstChild);
    }
    for (let responesline of resp){
        let li=document.createElement("LI");
        let t=responesline

        let p= document.createTextNode(responesline);
        let p2 = document.createElement("A");
        
        let delete_me_url="http://localhost:5000/files/"+responesline;
        li.appendChild(p);
        li.appendChild(p2);
        ReactDOM.render(e(DeleteButton,{xurl:delete_me_url}), p2);

        domContainer.appendChild(li);
    }
}



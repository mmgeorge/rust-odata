var searchIndex = {};
searchIndex["odata"] = {"doc":"Rust oData is library for building [OData][ODATA] REST services in Rust. This library is referenced and utilized by the [ConnectFour][CONNECTFOUR] project in the [game_server][GSERVE] repository for service generation.","items":[[0,"edm","odata","This module includes type information for a handful of Edm types. Notably the date types are not yet implemented. Type definition guidelines can be found [here][OASIS] OASIS: http://docs.oasis-open.org/odata/odata-json-csdl/v4.0/csprd01/odata-json-csdl-v4.0-csprd01.html#_Toc441572996",null,null],[4,"Type","odata::edm","",null,null],[13,"Binary","","",0,null],[13,"Boolean","","",0,null],[13,"Byte","","",0,null],[13,"Decimal","","",0,null],[13,"Double","","",0,null],[13,"Int16","","",0,null],[13,"Int32","","",0,null],[13,"Int64","","",0,null],[13,"String","","",0,null],[5,"from","","",null,{"inputs":[{"name":"str"}],"output":{"name":"type"}}],[5,"ty","","",null,{"inputs":[{"name":"type"}],"output":{"name":"vec"}}],[5,"format","","",null,{"inputs":[{"name":"type"}],"output":{"name":"str"}}],[0,"entity","odata","This module includes core functionality needed for declaring EntityTypes and EntitySets",null,null],[3,"EntityDescr","odata::entity","Internal type holding metadata for EntityTypes",null,null],[12,"name","","",1,null],[12,"keys","","",1,null],[12,"properties","","",1,null],[3,"Property","","Internal structure for holding property values for an Entity",null,null],[11,"name","","",1,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"keys","","",1,null],[11,"properties","","",1,null],[11,"new","","",2,{"inputs":[{"name":"str"},{"name":"type"}],"output":{"name":"property"}}],[11,"name","","",2,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"types","","",2,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"format","","",2,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[8,"Entity","","",null,null],[10,"describe","","Used to expose fields to model. Passed-up to Model through EntitySet",3,{"inputs":[],"output":{"name":"entitydescr"}}],[8,"EntitySet","","Trait for declaring CRUD-Q implementation.",null,null],[11,"create","","",4,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"res"}}],[11,"read","","",4,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"res"}}],[11,"read_list","","",4,{"inputs":[{"name":"self"}],"output":{"name":"res"}}],[11,"update","","",4,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"res"}}],[11,"delete","","",4,{"inputs":[{"name":"self"},{"name":"property"}],"output":{"name":"res"}}],[11,"query","","",4,{"inputs":[{"name":"self"},{"name":"vec"}],"output":{"name":"res"}}],[8,"EntitySetDescr","","Internal descriptor for an EntitySet for usage by the Model. Provides access to the underlying desciptor for the EntityType.",null,null],[10,"name","","",5,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[10,"descriptor","","",5,{"inputs":[{"name":"self"}],"output":{"name":"entitydescr"}}],[0,"model","odata","",null,null],[3,"Model","odata::model","Registry for storing metadata for each included EntitySet. Keys denote EntitySets, Values each EntitySets' respective properties.",null,null],[12,"actions","","",6,null],[3,"ModelBuilder","","",null,null],[6,"EsEntry","","Trait object for accessing both an EntitySet's descriptor and CRUD-Q implementation",null,null],[11,"lookup","","Lookup specified descriptor",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"option"}}],[11,"lookup_action","","",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"option"}}],[11,"render","","Renders the metadata document description of the oData Model",6,{"inputs":[{"name":"self"}],"output":null}],[11,"get_metadata","","Getter for the metadata document",6,{"inputs":[{"name":"self"}],"output":{"name":"value"}}],[11,"get_name","","Getter for model name",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"new","","",7,{"inputs":[{"name":"str"}],"output":{"name":"modelbuilder"}}],[11,"add","","Add a new EntitySet",7,{"inputs":[{"name":"self"},{"name":"e"}],"output":{"name":"modelbuilder"}}],[11,"action","","Add a new unbounded action",7,{"inputs":[{"name":"self"},{"name":"str"},{"name":"vec"},{"name":"f"}],"output":{"name":"modelbuilder"}}],[11,"build","","",7,{"inputs":[{"name":"self"}],"output":{"name":"model"}}],[0,"service","odata","Provides the wrapper around hyper that is used for creating an OData service. ",null,null],[3,"Service","odata::service","An instance of an oData service that will serve each model it contains. Constructed using a ServiceBuilder. ",null,null],[3,"ServiceBuilder","","Constructs an instance of a Service.",null,null],[4,"Error","","Error types for handler",null,null],[13,"NoImpl","","",8,null],[13,"InvalidRoot","","",8,null],[13,"InvalidModel","","",8,null],[13,"InvalidParameter","","",8,null],[13,"NotFound","","",8,null],[4,"Res","","Used for routing requests and for simplifying CRUD-Q implementation. Providing the correct HTTP StatusCode is handled by the server.",null,null],[13,"Succ","","",9,null],[13,"Created","","",9,null],[13,"Err","","",9,null],[11,"start","","Begin accepting requests.",10,{"inputs":[{"name":"self"}],"output":null}],[11,"new","","",11,{"inputs":[{"name":"str"}],"output":{"name":"servicebuilder"}}],[11,"add","","",11,{"inputs":[{"name":"self"},{"name":"model"}],"output":{"name":"self"}}],[11,"build","","",11,{"inputs":[{"name":"self"}],"output":{"name":"service"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[14,"rust_type","odata","",null,null],[14,"defEntity","","Declare a new EntityType, generating the metadata for use by the Model and Service class.",null,null],[14,"defEntitySet","","defEntitySet!(name: ident, entity_type: ident) Defines a macro for declaring EntitySets. For instance,  ``` defEntitySet!(Dogs, Dog);  ``` Declares an EntitySet named \"Dogs\" that consistes of elements of Entity Dog. CRUD-Q operations are then exposed via implementation of the EntitySet trait for struct. ",null,null],[11,"create","odata::entity","",4,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"res"}}],[11,"read","","",4,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"res"}}],[11,"read_list","","",4,{"inputs":[{"name":"self"}],"output":{"name":"res"}}],[11,"update","","",4,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"res"}}],[11,"delete","","",4,{"inputs":[{"name":"self"},{"name":"property"}],"output":{"name":"res"}}],[11,"query","","",4,{"inputs":[{"name":"self"},{"name":"vec"}],"output":{"name":"res"}}]],"paths":[[4,"Type"],[3,"EntityDescr"],[3,"Property"],[8,"Entity"],[8,"EntitySet"],[8,"EntitySetDescr"],[3,"Model"],[3,"ModelBuilder"],[4,"Error"],[4,"Res"],[3,"Service"],[3,"ServiceBuilder"]]};
initSearch(searchIndex);

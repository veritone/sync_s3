var searchIndex = JSON.parse('{\
"sync_s3":{"doc":"","t":[3,3,11,11,11,11,11,11,11,11,11,11,12,12,11,11,5,11,11,11,11,5,5,5,11,11,11,5,5,12,11,12,5,5,12,12,11,11,11,11,11,11,11,11,11],"n":["Args","Bucket","augment_args","augment_args_for_update","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","command","command_for_update","dest_profile","destination","fmt","fmt","format_key","from","from","from_arg_matches","from_arg_matches_mut","get_missing_keys","get_object","get_s3_client","group_id","into","into","list_bucket","main","name","new","path","put_object","run","source","src_profile","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","update_from_arg_matches","update_from_arg_matches_mut"],"q":["sync_s3","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Command line arguments for the executable","Parsed struct for s3 buckets passed to the program","","","","","","","","","","","The name of the destination s3 profile. Needs to be …","The name of the destination s3 bucket. Follows standard …","","","Formats a key and path in a usable way. Used primarily for …","Returns the argument unchanged.","Returns the argument unchanged.","","","Filters two lists of S3 objects to find ones missing in …","Retreives a given object from an S3 bucket as custom …","Returns a configured S3 client attached to the provided …","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns a result containing a vector of Strings detailing …","Retreives command line arguments, sets logging defaults, …","The name of the s3 bucket. Omits the s3:// part.","Returns a new Bucket struct using the input provided","The path added to the base name. Specifies the subfolder.","Uploads an object’s ByteStream to a specified S3 location","Connects to the S3 clients  using profiles, gets missing …","The name of the source s3 bucket. Follows standard s3://&lt;…","The name of the source s3 profile. Needs to be configured …","","","","","","","","",""],"i":[0,0,3,3,3,2,3,2,2,2,3,3,3,3,3,2,0,3,2,3,3,0,0,0,3,3,2,0,0,2,2,2,0,0,3,3,2,3,2,3,2,3,2,3,3],"f":[0,0,[1,1],[1,1],[[]],[[]],[[]],[[]],[2,2],[[]],[[],1],[[],1],0,0,[[3,4],5],[[2,4],5],[[],6],[[]],[[]],[7,[[9,[3,8]]]],[7,[[9,[3,8]]]],[[],[[9,[[10,[6]],[12,[11]]]]]],[[13,14,14],[[9,[15,16]]]],[14,[[9,[13,16]]]],[[],[[18,[17]]]],[[]],[[]],[[13,2],[[9,[[10,[6]],16]]]],[[]],0,[14,2],0,[[13,14,14,19],[[9,[20,16]]]],[3,[[9,[[12,[11]]]]]],0,0,[[]],[[],9],[[],9],[[],9],[[],9],[[],21],[[],21],[[3,7],[[9,[8]]]],[[3,7],[[9,[8]]]]],"p":[[3,"Command"],[3,"Bucket"],[3,"Args"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"ArgMatches"],[6,"Error"],[4,"Result"],[3,"Vec"],[8,"Error"],[3,"Box"],[3,"Client"],[15,"str"],[3,"GetObjectOutput"],[4,"Error"],[3,"Id"],[4,"Option"],[3,"ByteStream"],[3,"PutObjectOutput"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
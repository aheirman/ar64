import * as Rust from '$lib/socket_rust';

export async function PATCH({ request }) {
	/*let body   = Object.getOwnPropertySymbols(request).find(
        (s) => s.description === "state"
    ).body;
        for await (const v of body)
        console.log(v)
    */
    
	//console.log('path ' + request.url.pathname + ", body: " + request.body)
    console.log(await request.text())

    


    let status = 200;/*
	switch(request.url.pathname) {
		
        case '/api/ar64':
            {
                //console.log('/api/ar64', typeof request.body);
                //const src = JSON.parse(request.body).src;
                //console.log('src: ', src);
                try {
                    const response = await Rust.handleRust(request.body)
                    body = response;
                } catch(err) {
                    console.log("/api/ar64 errored" + err)
					//console.log('Bad api request: ', request);
                    body = err
                }
            }
            break;
        default:
            status = 404;
            console.log('Bad api request: ');
    }*/
	return new Response(body, {status: status})
  }
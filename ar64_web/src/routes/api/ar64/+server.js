export async function POST({ request }) {
    let request_body =  await request.text();
    console.log('API: "' + request_body + '"')

    


    let status = 200;
    let response_body;
    let response;
    try {
        /*const request = new Request('http://127.0.0.1:6379', {
            method: 'PATCH',
            body: body
        });
        console.log('serverjs: ' + request.body)
        const response = await fetch(request)*/
        
        response = await fetch('http://127.0.0.1:6379', {
            method: 'POST',
            cache: 'no-store',
            redirect: 'error',
            mode: "cors",
            body: request_body + "\r\n"//request_body
        })
    } catch(err) {
        console.log("/api/ar64 errored" + err)
        status = 404;
        response_body = err
        return new Response(response_body, {status: status})
    }
    response_body = await response.text();
    console.log('API: ' + request_body + ', finished, ' + response_body)
    
	return new Response(response_body, {status: status})
  }
export async function PATCH({ request }) {
    let body =  await request.text();
    console.log('API: ' + body)

    


    let status = 200;
    let response_body;
    try {
        const request = new Request('http://127.0.0.1:6379', {
            method: 'PATCH',
            body: body
        });
        const response = await fetch(request)
        response_body = response;
    } catch(err) {
        console.log("/api/ar64 errored" + err)
        //console.log('Bad api request: ', request);
        response_body = err
    }

	return new Response(response_body, {status: status})
  }
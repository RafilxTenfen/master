const express = require('express');
const app = express();

//gcp server
const Firestore = require('@google-cloud/firestore')
const db = new Firestore();

/* local server
const admin = require("firebase-admin");
const serviceAccount = require("./animalfeeder-81959-firebase-adminsdk-bgets-9411037a38.json");
admin.initializeApp({
    credential: admin.credential.cert(serviceAccount)
});
const db = admin.firestore();
*/

app.use(express.json());
const port = process.env.PORT || 8080;

app.listen(port, () => {
    console.log(`AnimalFeeder Rest API listening on port ${port}`);
});


app.get('/all', async (req, res) => {
    const query = db.collection('dispenser');
    const querySnapshot = await query.get();
    if (querySnapshot.size > 0) {
        res.json(querySnapshot.docs.map(doc => doc.data()));
    } else {
        res.json({status: 'Not found'});
    }
})

app.get('/img/:image', function(req,res){
    const image = req.params.image;
    res.sendFile(__dirname + '/img/' + image);
});

app.get('/', async (req, res) => {
    res.sendFile(__dirname + '/index.html');
})

app.post('/', async (req, res) => {
    try {
        await db.collection('dispenser').doc(req.body.id.toString()).set(req.body);
        res.json({ status: 'success', data: { dispenser: req.body } });
    } catch (e) {
        res.json({ status: 'error', data: { message: e.message, variables: {req_body: req.body} } });
    }
})
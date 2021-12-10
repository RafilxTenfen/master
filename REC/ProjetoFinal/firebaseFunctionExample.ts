// import * as functions from 'firebase-functions';
// import { OrganizationRepository } from '../repositories/organization_repository';

// export function generateJsonUsersReport(db: FirebaseFirestore.Firestore) {
//     return functions.https.onRequest(async (req: functions.https.Request, resp: functions.Response<any>) => {

//         console.log("---------------- Iniciar gerar relatório JSON de todos usuários de uma organização ----------------");

//         if (req.body["idProfessionalUser"] == undefined) {
//             resp.status(403).send({ "message": "Professional user not found" });
//         }
//         try {
//             const idProfessionalUser: string = req.body["idProfessionalUser"] as string;

//             const user = await db.collection('users').doc(idProfessionalUser).get();

//             const organizationDocumentReference: FirebaseFirestore.DocumentReference = user.data()?.organization;

//             try {

//                 const usersData = await OrganizationRepository.getAllOrganizationUserData(db, organizationDocumentReference);

//                 resp.header("Content-type", "application/json");
//                 resp.status(200).send(usersData);

//             } catch (error) {
//                 resp.status(400).send({ "message": "Unable to find the information" });
//             }
//         } catch (error) {
//             resp.status(400).send({ "message": "Professional user not found" });
//         }

//     }
//     );
// }
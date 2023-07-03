type Properties = {
    express_id: number; // expressID
    name: string;
    ifc_type: string;
    productcode: string;
    projectnummer: string;
    klant: string;
    station: string;
    bouwnummer: string;
    modulenaam: string;
    moduletype: string;
    categorie: string;
    materiaal: string;
    dikte: number;
    breedte: number;
    lengte: number;
    gewicht: number;
    volume: number;
    eenheid: string;
    ifc_bestand: string; // IFC Bestand
    aantal: number;
};

type ModelProperties = {
    model_id: number;
    properties: Properties[];
}
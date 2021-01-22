import React from 'react';
import { Button, Card, Grid, Message, Modal, Form, Label } from 'semantic-ui-react';

import KittyAvatar from './KittyAvatar';
import { TxButton } from './substrate-lib/components';

// --- About Modal ---

const TransferModal = props => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = React.useState(false);
  const [formValue, setFormValue] = React.useState({});

  const formChange = key => (ev, el) => {
    setFormValue(prev => ({
      ...prev,
      [key]: el.value
    }));  };

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>转让</Button>}>
    <Modal.Header>毛孩转让</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='转让对象' placeholder='对方地址' onChange={formChange('target')}/>
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认转让' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'transfer',
          inputParams: [formValue.target, kitty.id],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>;
};

// --- About Kitty Card ---

const KittyTitle = props =>{
  const {kitty, accountPair, setStatus} = props;
  if (kitty.owner == accountPair.address) {
    return (
        <Card.Content textAlign='right'>
          <Label style={{backgroundColor:'green'}}> {accountPair.meta.name.toUpperCase()}</Label>
        </Card.Content>
    )
  }else {
    return (<Card.Content textAlign='right'>
      <Label>  </Label>
    </Card.Content> )
  }
}

const KittyTransfer = props =>{
  const {kitty, accountPair, setStatus} = props;
  if (kitty.owner == accountPair.address) {
    return (
        <Card.Content extra textAlign='center'>
          <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
        </Card.Content>
    )
  }else {
    return (
        <Card.Content extra textAlign='center'>
          <Label>  </Label>
        </Card.Content>
    )
  }
}

const KittyCard = props => {
  const {kitty, accountPair, setStatus} = props;
  console.log(kitty.dna)
  return (
      <Card>
        <KittyTitle kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
        <Card.Content>
          <KittyAvatar dna={kitty.dna}/>
          <Card.Header>ID号：{kitty.id}</Card.Header>
          <Card.Meta style={{ overflowWrap: 'break-word' }}>基因：{kitty.dna.join(",")}</Card.Meta>
          <span style={{ overflowWrap: 'break-word' }}>猫奴：{kitty.owner}</span>
          <br/>
        </Card.Content>
        <KittyTransfer kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
      </Card>
  );
};

const KittyCards = props => {
    const { kitties, kittyOwners, kittyPrices, accountPair, setStatus } = props;
    const gridCss = {
        margin:"5px",
    };

    return <Grid columns='equal'><Grid.Row stretched>
        {
            kitties.map((kitty, index) => {
                return <Grid.Row key={index}>
                        <KittyCard kitty={kitty} owner={kittyOwners[index]} price={kittyPrices[index]} accountPair={accountPair} setStatus={setStatus} />
                    </Grid.Row>
            })
        }
    </Grid.Row>
    </Grid>;
};

export default KittyCards;

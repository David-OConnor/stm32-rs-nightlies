#[doc = "Register `I3C_DEVR0` reader"]
pub struct R(crate::R<I3C_DEVR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_DEVR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_DEVR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_DEVR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C_DEVR0` writer"]
pub struct W(crate::W<I3C_DEVR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_DEVR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I3C_DEVR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_DEVR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAVAL` reader - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
pub type DAVAL_R = crate::BitReader<bool>;
#[doc = "Field `DAVAL` writer - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
pub type DAVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR0_SPEC, bool, O>;
#[doc = "Field `DA` reader - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
pub type DA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DA` writer - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
pub type DA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_DEVR0_SPEC, u8, u8, 7, O>;
#[doc = "Field `IBIEN` reader - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
pub type IBIEN_R = crate::BitReader<bool>;
#[doc = "Field `IBIEN` writer - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
pub type IBIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR0_SPEC, bool, O>;
#[doc = "Field `CREN` reader - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
pub type CREN_R = crate::BitReader<bool>;
#[doc = "Field `CREN` writer - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
pub type CREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR0_SPEC, bool, O>;
#[doc = "Field `HJEN` reader - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
pub type HJEN_R = crate::BitReader<bool>;
#[doc = "Field `HJEN` writer - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
pub type HJEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR0_SPEC, bool, O>;
#[doc = "Field `AS` reader - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
pub type AS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTACT` reader - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
pub type RSTACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTVAL` reader - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
pub type RSTVAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
    #[inline(always)]
    pub fn daval(&self) -> DAVAL_R {
        DAVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
    #[inline(always)]
    pub fn ibien(&self) -> IBIEN_R {
        IBIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
    #[inline(always)]
    pub fn cren(&self) -> CREN_R {
        CREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
    #[inline(always)]
    pub fn hjen(&self) -> HJEN_R {
        HJEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
    #[inline(always)]
    pub fn rstact(&self) -> RSTACT_R {
        RSTACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
    #[inline(always)]
    pub fn rstval(&self) -> RSTVAL_R {
        RSTVAL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
    #[inline(always)]
    #[must_use]
    pub fn daval(&mut self) -> DAVAL_W<0> {
        DAVAL_W::new(self)
    }
    #[doc = "Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    #[doc = "Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
    #[inline(always)]
    #[must_use]
    pub fn ibien(&mut self) -> IBIEN_W<16> {
        IBIEN_W::new(self)
    }
    #[doc = "Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
    #[inline(always)]
    #[must_use]
    pub fn cren(&mut self) -> CREN_W<17> {
        CREN_W::new(self)
    }
    #[doc = "Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
    #[inline(always)]
    #[must_use]
    pub fn hjen(&mut self) -> HJEN_W<19> {
        HJEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C own device characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_devr0](index.html) module"]
pub struct I3C_DEVR0_SPEC;
impl crate::RegisterSpec for I3C_DEVR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_devr0::R](R) reader structure"]
impl crate::Readable for I3C_DEVR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c_devr0::W](W) writer structure"]
impl crate::Writable for I3C_DEVR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_DEVR0 to value 0"]
impl crate::Resettable for I3C_DEVR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

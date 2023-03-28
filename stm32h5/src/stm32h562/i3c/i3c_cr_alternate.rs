#[doc = "Register `I3C_CR_ALTERNATE` writer"]
pub struct W(crate::W<I3C_CR_ALTERNATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_CR_ALTERNATE_SPEC>;
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
impl From<crate::W<I3C_CR_ALTERNATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_CR_ALTERNATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCNT` writer - count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ..."]
pub type DCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I3C_CR_ALTERNATE_SPEC, u16, u16, 16, O>;
#[doc = "Field `CCC` writer - 8-bit CCC code (when I3C is acting as controller) If Bit\\[23\\]=CCC\\[7\\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\\[23\\]=CCC\\[7\\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0)."]
pub type CCC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_CR_ALTERNATE_SPEC, u8, u8, 8, O>;
#[doc = "Field `MTYPE` writer - message type (when I3C is acting as controller) Bits\\[23:16\\]
(CCC\\[7:0\\]) is the emitted 8-bit CCC code If Bit\\[23\\]=CCC\\[7\\]=1: this is the 1st part of an I3C SDR direct CCC command The transferred direct CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (direct CCC + T) + (8-bit Data + T)* + Sr After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). If Bit\\[23\\]=CCC\\[7\\]=0: this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0) The transferred broadcast CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (broadcast CCC + T) + (8-bit Data + T)* + Sr/P After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). others: reserved"]
pub type MTYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I3C_CR_ALTERNATE_SPEC, u8, u8, 4, O>;
#[doc = "Field `MEND` writer - message end type (when I3C is acting as controller)"]
pub type MEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_CR_ALTERNATE_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:15 - count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ..."]
    #[inline(always)]
    #[must_use]
    pub fn dcnt(&mut self) -> DCNT_W<0> {
        DCNT_W::new(self)
    }
    #[doc = "Bits 16:23 - 8-bit CCC code (when I3C is acting as controller) If Bit\\[23\\]=CCC\\[7\\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\\[23\\]=CCC\\[7\\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0)."]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CCC_W<16> {
        CCC_W::new(self)
    }
    #[doc = "Bits 27:30 - message type (when I3C is acting as controller) Bits\\[23:16\\]
(CCC\\[7:0\\]) is the emitted 8-bit CCC code If Bit\\[23\\]=CCC\\[7\\]=1: this is the 1st part of an I3C SDR direct CCC command The transferred direct CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (direct CCC + T) + (8-bit Data + T)* + Sr After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). If Bit\\[23\\]=CCC\\[7\\]=0: this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0) The transferred broadcast CCC command message is: {S / S+7’h7E +RnW=0 / Sr+*} + (broadcast CCC + T) + (8-bit Data + T)* + Sr/P After a S (START), depending on I3C_CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+R/W). others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mtype(&mut self) -> MTYPE_W<27> {
        MTYPE_W::new(self)
    }
    #[doc = "Bit 31 - message end type (when I3C is acting as controller)"]
    #[inline(always)]
    #[must_use]
    pub fn mend(&mut self) -> MEND_W<31> {
        MEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C message control register alternate\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_cr_alternate](index.html) module"]
pub struct I3C_CR_ALTERNATE_SPEC;
impl crate::RegisterSpec for I3C_CR_ALTERNATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [i3c_cr_alternate::W](W) writer structure"]
impl crate::Writable for I3C_CR_ALTERNATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_CR_ALTERNATE to value 0"]
impl crate::Resettable for I3C_CR_ALTERNATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

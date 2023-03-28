#[doc = "Register `MACVTR` reader"]
pub struct R(crate::R<MACVTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACVTR` writer"]
pub struct W(crate::W<MACVTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVTR_SPEC>;
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
impl From<crate::W<MACVTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Packets"]
pub type VL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Packets"]
pub type VL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACVTR_SPEC, u16, u16, 16, O>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_R = crate::BitReader<bool>;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type ETV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type VTIM_R = crate::BitReader<bool>;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type VTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub type ESVL_R = crate::BitReader<bool>;
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub type ESVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `ERSVLM` reader - Enable Receive S-VLAN Match"]
pub type ERSVLM_R = crate::BitReader<bool>;
#[doc = "Field `ERSVLM` writer - Enable Receive S-VLAN Match"]
pub type ERSVLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `DOVLTC` reader - Disable VLAN Type Check"]
pub type DOVLTC_R = crate::BitReader<bool>;
#[doc = "Field `DOVLTC` writer - Disable VLAN Type Check"]
pub type DOVLTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `EVLS` reader - Enable VLAN Tag Stripping on Receive"]
pub type EVLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVLS` writer - Enable VLAN Tag Stripping on Receive"]
pub type EVLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACVTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EVLRXS` reader - Enable VLAN Tag in Rx status"]
pub type EVLRXS_R = crate::BitReader<bool>;
#[doc = "Field `EVLRXS` writer - Enable VLAN Tag in Rx status"]
pub type EVLRXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub type VTHM_R = crate::BitReader<bool>;
#[doc = "Field `VTHM` writer - VLAN Tag Hash Table Match Enable"]
pub type VTHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `EDVLP` reader - Enable Double VLAN Processing"]
pub type EDVLP_R = crate::BitReader<bool>;
#[doc = "Field `EDVLP` writer - Enable Double VLAN Processing"]
pub type EDVLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `ERIVLT` reader - Enable Inner VLAN Tag"]
pub type ERIVLT_R = crate::BitReader<bool>;
#[doc = "Field `ERIVLT` writer - Enable Inner VLAN Tag"]
pub type ERIVLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
#[doc = "Field `EIVLS` reader - Enable Inner VLAN Tag Stripping on Receive"]
pub type EIVLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIVLS` writer - Enable Inner VLAN Tag Stripping on Receive"]
pub type EIVLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACVTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EIVLRXS` reader - Enable Inner VLAN Tag in Rx Status"]
pub type EIVLRXS_R = crate::BitReader<bool>;
#[doc = "Field `EIVLRXS` writer - Enable Inner VLAN Tag in Rx Status"]
pub type EIVLRXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check"]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing"]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag"]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<0> {
        VL_W::new(self)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<16> {
        ETV_W::new(self)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VTIM_W<17> {
        VTIM_W::new(self)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> ESVL_W<18> {
        ESVL_W::new(self)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match"]
    #[inline(always)]
    #[must_use]
    pub fn ersvlm(&mut self) -> ERSVLM_W<19> {
        ERSVLM_W::new(self)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check"]
    #[inline(always)]
    #[must_use]
    pub fn dovltc(&mut self) -> DOVLTC_W<20> {
        DOVLTC_W::new(self)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    #[must_use]
    pub fn evls(&mut self) -> EVLS_W<21> {
        EVLS_W::new(self)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status"]
    #[inline(always)]
    #[must_use]
    pub fn evlrxs(&mut self) -> EVLRXS_W<24> {
        EVLRXS_W::new(self)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vthm(&mut self) -> VTHM_W<25> {
        VTHM_W::new(self)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing"]
    #[inline(always)]
    #[must_use]
    pub fn edvlp(&mut self) -> EDVLP_W<26> {
        EDVLP_W::new(self)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag"]
    #[inline(always)]
    #[must_use]
    pub fn erivlt(&mut self) -> ERIVLT_W<27> {
        ERIVLT_W::new(self)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    #[must_use]
    pub fn eivls(&mut self) -> EIVLS_W<28> {
        EIVLS_W::new(self)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    #[must_use]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<31> {
        EIVLRXS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvtr](index.html) module"]
pub struct MACVTR_SPEC;
impl crate::RegisterSpec for MACVTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macvtr::R](R) reader structure"]
impl crate::Readable for MACVTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macvtr::W](W) writer structure"]
impl crate::Writable for MACVTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACVTR to value 0"]
impl crate::Resettable for MACVTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

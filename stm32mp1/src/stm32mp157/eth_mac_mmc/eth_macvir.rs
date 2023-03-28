#[doc = "Register `ETH_MACVIR` reader"]
pub struct R(crate::R<ETH_MACVIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACVIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACVIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACVIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACVIR` writer"]
pub struct W(crate::W<ETH_MACVIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACVIR_SPEC>;
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
impl From<crate::W<ETH_MACVIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACVIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLT` reader - VLT"]
pub type VLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLT` writer - VLT"]
pub type VLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACVIR_SPEC, u16, u16, 16, O>;
#[doc = "Field `VLC` reader - VLC"]
pub type VLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLC` writer - VLC"]
pub type VLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACVIR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VLP` reader - VLP"]
pub type VLP_R = crate::BitReader<bool>;
#[doc = "Field `VLP` writer - VLP"]
pub type VLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVIR_SPEC, bool, O>;
#[doc = "Field `CSVL` reader - CSVL"]
pub type CSVL_R = crate::BitReader<bool>;
#[doc = "Field `CSVL` writer - CSVL"]
pub type CSVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVIR_SPEC, bool, O>;
#[doc = "Field `VLTI` reader - VLTI"]
pub type VLTI_R = crate::BitReader<bool>;
#[doc = "Field `VLTI` writer - VLTI"]
pub type VLTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVIR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<0> {
        VLT_W::new(self)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<16> {
        VLC_W::new(self)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<18> {
        VLP_W::new(self)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<19> {
        CSVL_W::new(self)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<20> {
        VLTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The VLAN Tag Inclusion or Replacement register contains the VLAN tag for insertion or replacement in the Transmit packets. It also contains the VLAN tag insertion controls.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvir](index.html) module"]
pub struct ETH_MACVIR_SPEC;
impl crate::RegisterSpec for ETH_MACVIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macvir::R](R) reader structure"]
impl crate::Readable for ETH_MACVIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macvir::W](W) writer structure"]
impl crate::Writable for ETH_MACVIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACVIR to value 0"]
impl crate::Resettable for ETH_MACVIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

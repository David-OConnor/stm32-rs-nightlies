#[doc = "Register `MACIVIR` reader"]
pub struct R(crate::R<MACIVIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIVIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIVIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIVIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACIVIR` writer"]
pub struct W(crate::W<MACIVIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIVIR_SPEC>;
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
impl From<crate::W<MACIVIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIVIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Packets"]
pub type VLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Packets"]
pub type VLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACIVIR_SPEC, u16, u16, 16, O>;
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Packets"]
pub type VLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Packets"]
pub type VLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACIVIR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VLP` reader - VLAN Priority Control"]
pub type VLP_R = crate::BitReader<bool>;
#[doc = "Field `VLP` writer - VLAN Priority Control"]
pub type VLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIVIR_SPEC, bool, O>;
#[doc = "Field `CSVL` reader - C-VLAN or S-VLAN"]
pub type CSVL_R = crate::BitReader<bool>;
#[doc = "Field `CSVL` writer - C-VLAN or S-VLAN"]
pub type CSVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIVIR_SPEC, bool, O>;
#[doc = "Field `VLTI` reader - VLAN Tag Input"]
pub type VLTI_R = crate::BitReader<bool>;
#[doc = "Field `VLTI` writer - VLAN Tag Input"]
pub type VLTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIVIR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Packets"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Packets"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLAN Tag Input"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Packets"]
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<0> {
        VLT_W::new(self)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Packets"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<16> {
        VLC_W::new(self)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<18> {
        VLP_W::new(self)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<19> {
        CSVL_W::new(self)
    }
    #[doc = "Bit 20 - VLAN Tag Input"]
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
#[doc = "Inner VLAN inclusion register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macivir](index.html) module"]
pub struct MACIVIR_SPEC;
impl crate::RegisterSpec for MACIVIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macivir::R](R) reader structure"]
impl crate::Readable for MACIVIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macivir::W](W) writer structure"]
impl crate::Writable for MACIVIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACIVIR to value 0"]
impl crate::Resettable for MACIVIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

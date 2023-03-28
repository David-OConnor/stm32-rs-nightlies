#[doc = "Register `OPTCR2` reader"]
pub struct R(crate::R<OPTCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR2` writer"]
pub struct W(crate::W<OPTCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR2_SPEC>;
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
impl From<crate::W<OPTCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROPi` reader - PCROP option byte"]
pub type PCROPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCROPi` writer - PCROP option byte"]
pub type PCROPI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PCROP_RDP` reader - PCROP zone preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader<bool>;
#[doc = "Field `PCROP_RDP` writer - PCROP zone preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    pub fn pcropi(&self) -> PCROPI_R {
        PCROPI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    #[must_use]
    pub fn pcropi(&mut self) -> PCROPI_W<0> {
        PCROPI_W::new(self)
    }
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<31> {
        PCROP_RDP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr2](index.html) module"]
pub struct OPTCR2_SPEC;
impl crate::RegisterSpec for OPTCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr2::R](R) reader structure"]
impl crate::Readable for OPTCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr2::W](W) writer structure"]
impl crate::Writable for OPTCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTCR2 to value 0x8000_00ff"]
impl crate::Resettable for OPTCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_00ff;
}

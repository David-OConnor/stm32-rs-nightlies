#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUDIS` reader - Master Update Disable"]
pub type MUDIS_R = crate::BitReader<bool>;
#[doc = "Field `MUDIS` writer - Master Update Disable"]
pub type MUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TAUDIS` reader - Timer A Update Disable"]
pub type TAUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TAUDIS` writer - Timer A Update Disable"]
pub type TAUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TBUDIS` reader - Timer B Update Disable"]
pub type TBUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TBUDIS` writer - Timer B Update Disable"]
pub type TBUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TCUDIS` reader - Timer C Update Disable"]
pub type TCUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TCUDIS` writer - Timer C Update Disable"]
pub type TCUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TDUDIS` reader - Timer D Update Disable"]
pub type TDUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TDUDIS` writer - Timer D Update Disable"]
pub type TDUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TEUDIS` reader - Timer E Update Disable"]
pub type TEUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TEUDIS` writer - Timer E Update Disable"]
pub type TEUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TFUDIS` reader - Timer f Update Disable"]
pub type TFUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TFUDIS` writer - Timer f Update Disable"]
pub type TFUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AD1USRC` reader - ADC Trigger 1 Update Source"]
pub type AD1USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD1USRC` writer - ADC Trigger 1 Update Source"]
pub type AD1USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD2USRC` reader - ADC Trigger 2 Update Source"]
pub type AD2USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD2USRC` writer - ADC Trigger 2 Update Source"]
pub type AD2USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD3USRC` reader - ADC Trigger 3 Update Source"]
pub type AD3USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD3USRC` writer - ADC Trigger 3 Update Source"]
pub type AD3USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `AD4USRC` reader - ADC Trigger 4 Update Source"]
pub type AD4USRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD4USRC` writer - ADC Trigger 4 Update Source"]
pub type AD4USRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    pub fn mudis(&self) -> MUDIS_R {
        MUDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    pub fn taudis(&self) -> TAUDIS_R {
        TAUDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    pub fn tbudis(&self) -> TBUDIS_R {
        TBUDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    pub fn tcudis(&self) -> TCUDIS_R {
        TCUDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    pub fn tdudis(&self) -> TDUDIS_R {
        TDUDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    pub fn teudis(&self) -> TEUDIS_R {
        TEUDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer f Update Disable"]
    #[inline(always)]
    pub fn tfudis(&self) -> TFUDIS_R {
        TFUDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    pub fn ad1usrc(&self) -> AD1USRC_R {
        AD1USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    pub fn ad2usrc(&self) -> AD2USRC_R {
        AD2USRC_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    pub fn ad3usrc(&self) -> AD3USRC_R {
        AD3USRC_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    pub fn ad4usrc(&self) -> AD4USRC_R {
        AD4USRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mudis(&mut self) -> MUDIS_W<0> {
        MUDIS_W::new(self)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn taudis(&mut self) -> TAUDIS_W<1> {
        TAUDIS_W::new(self)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tbudis(&mut self) -> TBUDIS_W<2> {
        TBUDIS_W::new(self)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tcudis(&mut self) -> TCUDIS_W<3> {
        TCUDIS_W::new(self)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdudis(&mut self) -> TDUDIS_W<4> {
        TDUDIS_W::new(self)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn teudis(&mut self) -> TEUDIS_W<5> {
        TEUDIS_W::new(self)
    }
    #[doc = "Bit 6 - Timer f Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tfudis(&mut self) -> TFUDIS_W<6> {
        TFUDIS_W::new(self)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad1usrc(&mut self) -> AD1USRC_W<16> {
        AD1USRC_W::new(self)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad2usrc(&mut self) -> AD2USRC_W<19> {
        AD2USRC_W::new(self)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad3usrc(&mut self) -> AD3USRC_W<22> {
        AD3USRC_W::new(self)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad4usrc(&mut self) -> AD4USRC_W<25> {
        AD4USRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

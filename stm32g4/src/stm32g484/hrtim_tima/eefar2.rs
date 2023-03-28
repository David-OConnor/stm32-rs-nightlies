#[doc = "Register `EEFAR2` reader"]
pub struct R(crate::R<EEFAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFAR2` writer"]
pub struct W(crate::W<EEFAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFAR2_SPEC>;
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
impl From<crate::W<EEFAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6LTCH` reader - External Event 6 latch"]
pub type EE6LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE6LTCH` writer - External Event 6 latch"]
pub type EE6LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, bool, O>;
#[doc = "Field `EE6FLTR` reader - External Event 6 filter"]
pub type EE6FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE6FLTR` writer - External Event 6 filter"]
pub type EE6FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFAR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE7LTCH` reader - External Event 7 latch"]
pub type EE7LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE7LTCH` writer - External Event 7 latch"]
pub type EE7LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, bool, O>;
#[doc = "Field `EE7FLTR` reader - External Event 7 filter"]
pub type EE7FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE7FLTR` writer - External Event 7 filter"]
pub type EE7FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFAR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE8LTCH` reader - External Event 8 latch"]
pub type EE8LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE8LTCH` writer - External Event 8 latch"]
pub type EE8LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, bool, O>;
#[doc = "Field `EE8FLTR` reader - External Event 8 filter"]
pub type EE8FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE8FLTR` writer - External Event 8 filter"]
pub type EE8FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFAR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE9LTCH` reader - External Event 9 latch"]
pub type EE9LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE9LTCH` writer - External Event 9 latch"]
pub type EE9LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, bool, O>;
#[doc = "Field `EE9FLTR` reader - External Event 9 filter"]
pub type EE9FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE9FLTR` writer - External Event 9 filter"]
pub type EE9FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFAR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE10LTCH` reader - External Event 10 latch"]
pub type EE10LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE10LTCH` writer - External Event 10 latch"]
pub type EE10LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, bool, O>;
#[doc = "Field `EE10FLTR` reader - External Event 10 filter"]
pub type EE10FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE10FLTR` writer - External Event 10 filter"]
pub type EE10FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFAR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<0> {
        EE6LTCH_W::new(self)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<1> {
        EE6FLTR_W::new(self)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<6> {
        EE7LTCH_W::new(self)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<7> {
        EE7FLTR_W::new(self)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<12> {
        EE8LTCH_W::new(self)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<13> {
        EE8FLTR_W::new(self)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<18> {
        EE9LTCH_W::new(self)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<19> {
        EE9FLTR_W::new(self)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<24> {
        EE10LTCH_W::new(self)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<25> {
        EE10FLTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefar2](index.html) module"]
pub struct EEFAR2_SPEC;
impl crate::RegisterSpec for EEFAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefar2::R](R) reader structure"]
impl crate::Readable for EEFAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefar2::W](W) writer structure"]
impl crate::Writable for EEFAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEFAR2 to value 0"]
impl crate::Resettable for EEFAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

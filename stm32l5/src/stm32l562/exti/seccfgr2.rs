#[doc = "Register `SECCFGR2` reader"]
pub struct R(crate::R<SECCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR2` writer"]
pub struct W(crate::W<SECCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR2_SPEC>;
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
impl From<crate::W<SECCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC32` reader - SEC32"]
pub type SEC32_R = crate::BitReader<bool>;
#[doc = "Field `SEC32` writer - SEC32"]
pub type SEC32_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC33` reader - SEC33"]
pub type SEC33_R = crate::BitReader<bool>;
#[doc = "Field `SEC33` writer - SEC33"]
pub type SEC33_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC34` reader - SEC34"]
pub type SEC34_R = crate::BitReader<bool>;
#[doc = "Field `SEC34` writer - SEC34"]
pub type SEC34_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC35` reader - SEC35"]
pub type SEC35_R = crate::BitReader<bool>;
#[doc = "Field `SEC35` writer - SEC35"]
pub type SEC35_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC36` reader - SEC36"]
pub type SEC36_R = crate::BitReader<bool>;
#[doc = "Field `SEC36` writer - SEC36"]
pub type SEC36_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC37` reader - SEC37"]
pub type SEC37_R = crate::BitReader<bool>;
#[doc = "Field `SEC37` writer - SEC37"]
pub type SEC37_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC38` reader - SEC38"]
pub type SEC38_R = crate::BitReader<bool>;
#[doc = "Field `SEC38` writer - SEC38"]
pub type SEC38_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC39` reader - SEC39"]
pub type SEC39_R = crate::BitReader<bool>;
#[doc = "Field `SEC39` writer - SEC39"]
pub type SEC39_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC40` reader - SEC40"]
pub type SEC40_R = crate::BitReader<bool>;
#[doc = "Field `SEC40` writer - SEC40"]
pub type SEC40_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC41` reader - SEC41"]
pub type SEC41_R = crate::BitReader<bool>;
#[doc = "Field `SEC41` writer - SEC41"]
pub type SEC41_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
#[doc = "Field `SEC42` reader - SEC42"]
pub type SEC42_R = crate::BitReader<bool>;
#[doc = "Field `SEC42` writer - SEC42"]
pub type SEC42_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SEC32"]
    #[inline(always)]
    pub fn sec32(&self) -> SEC32_R {
        SEC32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SEC33"]
    #[inline(always)]
    pub fn sec33(&self) -> SEC33_R {
        SEC33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEC34"]
    #[inline(always)]
    pub fn sec34(&self) -> SEC34_R {
        SEC34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SEC35"]
    #[inline(always)]
    pub fn sec35(&self) -> SEC35_R {
        SEC35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEC36"]
    #[inline(always)]
    pub fn sec36(&self) -> SEC36_R {
        SEC36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEC37"]
    #[inline(always)]
    pub fn sec37(&self) -> SEC37_R {
        SEC37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SEC38"]
    #[inline(always)]
    pub fn sec38(&self) -> SEC38_R {
        SEC38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SEC39"]
    #[inline(always)]
    pub fn sec39(&self) -> SEC39_R {
        SEC39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SEC40"]
    #[inline(always)]
    pub fn sec40(&self) -> SEC40_R {
        SEC40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SEC41"]
    #[inline(always)]
    pub fn sec41(&self) -> SEC41_R {
        SEC41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEC42"]
    #[inline(always)]
    pub fn sec42(&self) -> SEC42_R {
        SEC42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEC32"]
    #[inline(always)]
    #[must_use]
    pub fn sec32(&mut self) -> SEC32_W<0> {
        SEC32_W::new(self)
    }
    #[doc = "Bit 1 - SEC33"]
    #[inline(always)]
    #[must_use]
    pub fn sec33(&mut self) -> SEC33_W<1> {
        SEC33_W::new(self)
    }
    #[doc = "Bit 2 - SEC34"]
    #[inline(always)]
    #[must_use]
    pub fn sec34(&mut self) -> SEC34_W<2> {
        SEC34_W::new(self)
    }
    #[doc = "Bit 3 - SEC35"]
    #[inline(always)]
    #[must_use]
    pub fn sec35(&mut self) -> SEC35_W<3> {
        SEC35_W::new(self)
    }
    #[doc = "Bit 4 - SEC36"]
    #[inline(always)]
    #[must_use]
    pub fn sec36(&mut self) -> SEC36_W<4> {
        SEC36_W::new(self)
    }
    #[doc = "Bit 5 - SEC37"]
    #[inline(always)]
    #[must_use]
    pub fn sec37(&mut self) -> SEC37_W<5> {
        SEC37_W::new(self)
    }
    #[doc = "Bit 6 - SEC38"]
    #[inline(always)]
    #[must_use]
    pub fn sec38(&mut self) -> SEC38_W<6> {
        SEC38_W::new(self)
    }
    #[doc = "Bit 7 - SEC39"]
    #[inline(always)]
    #[must_use]
    pub fn sec39(&mut self) -> SEC39_W<7> {
        SEC39_W::new(self)
    }
    #[doc = "Bit 8 - SEC40"]
    #[inline(always)]
    #[must_use]
    pub fn sec40(&mut self) -> SEC40_W<8> {
        SEC40_W::new(self)
    }
    #[doc = "Bit 9 - SEC41"]
    #[inline(always)]
    #[must_use]
    pub fn sec41(&mut self) -> SEC41_W<9> {
        SEC41_W::new(self)
    }
    #[doc = "Bit 10 - SEC42"]
    #[inline(always)]
    #[must_use]
    pub fn sec42(&mut self) -> SEC42_W<10> {
        SEC42_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI security enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr2](index.html) module"]
pub struct SECCFGR2_SPEC;
impl crate::RegisterSpec for SECCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr2::R](R) reader structure"]
impl crate::Readable for SECCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr2::W](W) writer structure"]
impl crate::Writable for SECCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR2 to value 0"]
impl crate::Resettable for SECCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

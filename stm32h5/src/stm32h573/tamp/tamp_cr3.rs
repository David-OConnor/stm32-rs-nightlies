#[doc = "Register `TAMP_CR3` reader"]
pub struct R(crate::R<TAMP_CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_CR3` writer"]
pub struct W(crate::W<TAMP_CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_CR3_SPEC>;
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
impl From<crate::W<TAMP_CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITAMP1NOER` reader - Internal Tamper 1 no erase"]
pub type ITAMP1NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP1NOER` writer - Internal Tamper 1 no erase"]
pub type ITAMP1NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP2NOER` reader - Internal Tamper 2 no erase"]
pub type ITAMP2NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP2NOER` writer - Internal Tamper 2 no erase"]
pub type ITAMP2NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP3NOER` reader - Internal Tamper 3 no erase"]
pub type ITAMP3NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP3NOER` writer - Internal Tamper 3 no erase"]
pub type ITAMP3NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP4NOER` reader - Internal Tamper 4 no erase"]
pub type ITAMP4NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP4NOER` writer - Internal Tamper 4 no erase"]
pub type ITAMP4NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP5NOER` reader - Internal Tamper 5 no erase"]
pub type ITAMP5NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP5NOER` writer - Internal Tamper 5 no erase"]
pub type ITAMP5NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP6NOER` reader - Internal Tamper 6 no erase"]
pub type ITAMP6NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP6NOER` writer - Internal Tamper 6 no erase"]
pub type ITAMP6NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP7NOER` reader - Internal Tamper 7 no erase"]
pub type ITAMP7NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP7NOER` writer - Internal Tamper 7 no erase"]
pub type ITAMP7NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP8NOER` reader - Internal Tamper 8 no erase"]
pub type ITAMP8NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP8NOER` writer - Internal Tamper 8 no erase"]
pub type ITAMP8NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP9NOER` reader - Internal Tamper 9 no erase"]
pub type ITAMP9NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP9NOER` writer - Internal Tamper 9 no erase"]
pub type ITAMP9NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP11NOER` reader - Internal Tamper 11 no erase"]
pub type ITAMP11NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP11NOER` writer - Internal Tamper 11 no erase"]
pub type ITAMP11NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP12NOER` reader - Internal Tamper 12 no erase"]
pub type ITAMP12NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP12NOER` writer - Internal Tamper 12 no erase"]
pub type ITAMP12NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP13NOER` reader - Internal Tamper 13 no erase"]
pub type ITAMP13NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP13NOER` writer - Internal Tamper 13 no erase"]
pub type ITAMP13NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
#[doc = "Field `ITAMP15NOER` reader - Internal Tamper 15 no erase"]
pub type ITAMP15NOER_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP15NOER` writer - Internal Tamper 15 no erase"]
pub type ITAMP15NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_CR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Internal Tamper 1 no erase"]
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Tamper 2 no erase"]
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Tamper 3 no erase"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Tamper 4 no erase"]
    #[inline(always)]
    pub fn itamp4noer(&self) -> ITAMP4NOER_R {
        ITAMP4NOER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Internal Tamper 5 no erase"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal Tamper 6 no erase"]
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal Tamper 7 no erase"]
    #[inline(always)]
    pub fn itamp7noer(&self) -> ITAMP7NOER_R {
        ITAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal Tamper 8 no erase"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal Tamper 9 no erase"]
    #[inline(always)]
    pub fn itamp9noer(&self) -> ITAMP9NOER_R {
        ITAMP9NOER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Internal Tamper 11 no erase"]
    #[inline(always)]
    pub fn itamp11noer(&self) -> ITAMP11NOER_R {
        ITAMP11NOER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Internal Tamper 12 no erase"]
    #[inline(always)]
    pub fn itamp12noer(&self) -> ITAMP12NOER_R {
        ITAMP12NOER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal Tamper 13 no erase"]
    #[inline(always)]
    pub fn itamp13noer(&self) -> ITAMP13NOER_R {
        ITAMP13NOER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal Tamper 15 no erase"]
    #[inline(always)]
    pub fn itamp15noer(&self) -> ITAMP15NOER_R {
        ITAMP15NOER_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Tamper 1 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<0> {
        ITAMP1NOER_W::new(self)
    }
    #[doc = "Bit 1 - Internal Tamper 2 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<1> {
        ITAMP2NOER_W::new(self)
    }
    #[doc = "Bit 2 - Internal Tamper 3 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<2> {
        ITAMP3NOER_W::new(self)
    }
    #[doc = "Bit 3 - Internal Tamper 4 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4noer(&mut self) -> ITAMP4NOER_W<3> {
        ITAMP4NOER_W::new(self)
    }
    #[doc = "Bit 4 - Internal Tamper 5 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<4> {
        ITAMP5NOER_W::new(self)
    }
    #[doc = "Bit 5 - Internal Tamper 6 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W<5> {
        ITAMP6NOER_W::new(self)
    }
    #[doc = "Bit 6 - Internal Tamper 7 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp7noer(&mut self) -> ITAMP7NOER_W<6> {
        ITAMP7NOER_W::new(self)
    }
    #[doc = "Bit 7 - Internal Tamper 8 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<7> {
        ITAMP8NOER_W::new(self)
    }
    #[doc = "Bit 8 - Internal Tamper 9 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp9noer(&mut self) -> ITAMP9NOER_W<8> {
        ITAMP9NOER_W::new(self)
    }
    #[doc = "Bit 10 - Internal Tamper 11 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp11noer(&mut self) -> ITAMP11NOER_W<10> {
        ITAMP11NOER_W::new(self)
    }
    #[doc = "Bit 11 - Internal Tamper 12 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp12noer(&mut self) -> ITAMP12NOER_W<11> {
        ITAMP12NOER_W::new(self)
    }
    #[doc = "Bit 12 - Internal Tamper 13 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp13noer(&mut self) -> ITAMP13NOER_W<12> {
        ITAMP13NOER_W::new(self)
    }
    #[doc = "Bit 14 - Internal Tamper 15 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp15noer(&mut self) -> ITAMP15NOER_W<14> {
        ITAMP15NOER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_cr3](index.html) module"]
pub struct TAMP_CR3_SPEC;
impl crate::RegisterSpec for TAMP_CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_cr3::R](R) reader structure"]
impl crate::Readable for TAMP_CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_cr3::W](W) writer structure"]
impl crate::Writable for TAMP_CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_CR3 to value 0"]
impl crate::Resettable for TAMP_CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

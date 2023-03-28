#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1NOER` reader - TAMP1NOER"]
pub type TAMP1NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1NOER` writer - TAMP1NOER"]
pub type TAMP1NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP2NOER` reader - TAMP2NOER"]
pub type TAMP2NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2NOER` writer - TAMP2NOER"]
pub type TAMP2NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP3NOER` reader - TAMP3NOER"]
pub type TAMP3NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3NOER` writer - TAMP3NOER"]
pub type TAMP3NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP4NOER` reader - TAMP4NOER"]
pub type TAMP4NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP4NOER` writer - TAMP4NOER"]
pub type TAMP4NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP5NOER` reader - TAMP5NOER"]
pub type TAMP5NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP5NOER` writer - TAMP5NOER"]
pub type TAMP5NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP6NOER` reader - TAMP6NOER"]
pub type TAMP6NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP6NOER` writer - TAMP6NOER"]
pub type TAMP6NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP7NOER` reader - TAMP7NOER"]
pub type TAMP7NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP7NOER` writer - TAMP7NOER"]
pub type TAMP7NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP8NOER` reader - TAMP8NOER"]
pub type TAMP8NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP8NOER` writer - TAMP8NOER"]
pub type TAMP8NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP1MSK` reader - TAMP1MSK"]
pub type TAMP1MSK_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1MSK` writer - TAMP1MSK"]
pub type TAMP1MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP2MSK` reader - TAMP2MSK"]
pub type TAMP2MSK_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2MSK` writer - TAMP2MSK"]
pub type TAMP2MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP3MSK` reader - TAMP3MSK"]
pub type TAMP3MSK_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3MSK` writer - TAMP3MSK"]
pub type TAMP3MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `BKERASE` reader - BKERASE"]
pub type BKERASE_R = crate::BitReader<bool>;
#[doc = "Field `BKERASE` writer - BKERASE"]
pub type BKERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP1TRG` reader - TAMP1TRG"]
pub type TAMP1TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1TRG` writer - TAMP1TRG"]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP2TRG` reader - TAMP2TRG"]
pub type TAMP2TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2TRG` writer - TAMP2TRG"]
pub type TAMP2TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP3TRG` reader - TAMP3TRG"]
pub type TAMP3TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3TRG` writer - TAMP3TRG"]
pub type TAMP3TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP4TRG` reader - TAMP4TRG"]
pub type TAMP4TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP4TRG` writer - TAMP4TRG"]
pub type TAMP4TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP5TRG` reader - TAMP5TRG"]
pub type TAMP5TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP5TRG` writer - TAMP5TRG"]
pub type TAMP5TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP6TRG` reader - TAMP6TRG"]
pub type TAMP6TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP6TRG` writer - TAMP6TRG"]
pub type TAMP6TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP7TRG` reader - TAMP7TRG"]
pub type TAMP7TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP7TRG` writer - TAMP7TRG"]
pub type TAMP7TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP8TRG` reader - TAMP8TRG"]
pub type TAMP8TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP8TRG` writer - TAMP8TRG"]
pub type TAMP8TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TAMP4NOER"]
    #[inline(always)]
    pub fn tamp4noer(&self) -> TAMP4NOER_R {
        TAMP4NOER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TAMP5NOER"]
    #[inline(always)]
    pub fn tamp5noer(&self) -> TAMP5NOER_R {
        TAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TAMP6NOER"]
    #[inline(always)]
    pub fn tamp6noer(&self) -> TAMP6NOER_R {
        TAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TAMP7NOER"]
    #[inline(always)]
    pub fn tamp7noer(&self) -> TAMP7NOER_R {
        TAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TAMP8NOER"]
    #[inline(always)]
    pub fn tamp8noer(&self) -> TAMP8NOER_R {
        TAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - BKERASE"]
    #[inline(always)]
    pub fn bkerase(&self) -> BKERASE_R {
        BKERASE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TAMP4TRG"]
    #[inline(always)]
    pub fn tamp4trg(&self) -> TAMP4TRG_R {
        TAMP4TRG_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TAMP5TRG"]
    #[inline(always)]
    pub fn tamp5trg(&self) -> TAMP5TRG_R {
        TAMP5TRG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMP6TRG"]
    #[inline(always)]
    pub fn tamp6trg(&self) -> TAMP6TRG_R {
        TAMP6TRG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMP7TRG"]
    #[inline(always)]
    pub fn tamp7trg(&self) -> TAMP7TRG_R {
        TAMP7TRG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TAMP8TRG"]
    #[inline(always)]
    pub fn tamp8trg(&self) -> TAMP8TRG_R {
        TAMP8TRG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<0> {
        TAMP1NOER_W::new(self)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<1> {
        TAMP2NOER_W::new(self)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W<2> {
        TAMP3NOER_W::new(self)
    }
    #[doc = "Bit 3 - TAMP4NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4noer(&mut self) -> TAMP4NOER_W<3> {
        TAMP4NOER_W::new(self)
    }
    #[doc = "Bit 4 - TAMP5NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5noer(&mut self) -> TAMP5NOER_W<4> {
        TAMP5NOER_W::new(self)
    }
    #[doc = "Bit 5 - TAMP6NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp6noer(&mut self) -> TAMP6NOER_W<5> {
        TAMP6NOER_W::new(self)
    }
    #[doc = "Bit 6 - TAMP7NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp7noer(&mut self) -> TAMP7NOER_W<6> {
        TAMP7NOER_W::new(self)
    }
    #[doc = "Bit 7 - TAMP8NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp8noer(&mut self) -> TAMP8NOER_W<7> {
        TAMP8NOER_W::new(self)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<16> {
        TAMP1MSK_W::new(self)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<17> {
        TAMP2MSK_W::new(self)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W<18> {
        TAMP3MSK_W::new(self)
    }
    #[doc = "Bit 23 - BKERASE"]
    #[inline(always)]
    #[must_use]
    pub fn bkerase(&mut self) -> BKERASE_W<23> {
        BKERASE_W::new(self)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<24> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<25> {
        TAMP2TRG_W::new(self)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<26> {
        TAMP3TRG_W::new(self)
    }
    #[doc = "Bit 27 - TAMP4TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4trg(&mut self) -> TAMP4TRG_W<27> {
        TAMP4TRG_W::new(self)
    }
    #[doc = "Bit 28 - TAMP5TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5trg(&mut self) -> TAMP5TRG_W<28> {
        TAMP5TRG_W::new(self)
    }
    #[doc = "Bit 29 - TAMP6TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp6trg(&mut self) -> TAMP6TRG_W<29> {
        TAMP6TRG_W::new(self)
    }
    #[doc = "Bit 30 - TAMP7TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp7trg(&mut self) -> TAMP7TRG_W<30> {
        TAMP7TRG_W::new(self)
    }
    #[doc = "Bit 31 - TAMP8TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp8trg(&mut self) -> TAMP8TRG_W<31> {
        TAMP8TRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

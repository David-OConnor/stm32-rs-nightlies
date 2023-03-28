#[doc = "Register `ASCR2` reader"]
pub struct R(crate::R<ASCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCR2` writer"]
pub struct W(crate::W<ASCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCR2_SPEC>;
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
impl From<crate::W<ASCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GR10_1` reader - GR10_1 analog switch control"]
pub type GR10_1_R = crate::BitReader<bool>;
#[doc = "Field `GR10_1` writer - GR10_1 analog switch control"]
pub type GR10_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR10_2` reader - GR10_2 analog switch control"]
pub type GR10_2_R = crate::BitReader<bool>;
#[doc = "Field `GR10_2` writer - GR10_2 analog switch control"]
pub type GR10_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR10_3` reader - GR10_3 analog switch control"]
pub type GR10_3_R = crate::BitReader<bool>;
#[doc = "Field `GR10_3` writer - GR10_3 analog switch control"]
pub type GR10_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR10_4` reader - GR10_4 analog switch control"]
pub type GR10_4_R = crate::BitReader<bool>;
#[doc = "Field `GR10_4` writer - GR10_4 analog switch control"]
pub type GR10_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR6_1` reader - GR6_1 analog switch control"]
pub type GR6_1_R = crate::BitReader<bool>;
#[doc = "Field `GR6_1` writer - GR6_1 analog switch control"]
pub type GR6_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR6_2` reader - GR6_2 analog switch control"]
pub type GR6_2_R = crate::BitReader<bool>;
#[doc = "Field `GR6_2` writer - GR6_2 analog switch control"]
pub type GR6_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR5_1` reader - GR5_1 analog switch control"]
pub type GR5_1_R = crate::BitReader<bool>;
#[doc = "Field `GR5_1` writer - GR5_1 analog switch control"]
pub type GR5_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR5_2` reader - GR5_2 analog switch control"]
pub type GR5_2_R = crate::BitReader<bool>;
#[doc = "Field `GR5_2` writer - GR5_2 analog switch control"]
pub type GR5_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR5_3` reader - GR5_3 analog switch control"]
pub type GR5_3_R = crate::BitReader<bool>;
#[doc = "Field `GR5_3` writer - GR5_3 analog switch control"]
pub type GR5_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR4_1` reader - GR4_1 analog switch control"]
pub type GR4_1_R = crate::BitReader<bool>;
#[doc = "Field `GR4_1` writer - GR4_1 analog switch control"]
pub type GR4_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR4_2` reader - GR4_2 analog switch control"]
pub type GR4_2_R = crate::BitReader<bool>;
#[doc = "Field `GR4_2` writer - GR4_2 analog switch control"]
pub type GR4_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR4_3` reader - GR4_3 analog switch control"]
pub type GR4_3_R = crate::BitReader<bool>;
#[doc = "Field `GR4_3` writer - GR4_3 analog switch control"]
pub type GR4_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR3_3` reader - GR3_3 analog switch control"]
pub type GR3_3_R = crate::BitReader<bool>;
#[doc = "Field `GR3_3` writer - GR3_3 analog switch control"]
pub type GR3_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR3_4` reader - GR3_4 analog switch control"]
pub type GR3_4_R = crate::BitReader<bool>;
#[doc = "Field `GR3_4` writer - GR3_4 analog switch control"]
pub type GR3_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR3_5` reader - GR3_5 analog switch control"]
pub type GR3_5_R = crate::BitReader<bool>;
#[doc = "Field `GR3_5` writer - GR3_5 analog switch control"]
pub type GR3_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR9_3` reader - GR9_3 analog switch control"]
pub type GR9_3_R = crate::BitReader<bool>;
#[doc = "Field `GR9_3` writer - GR9_3 analog switch control"]
pub type GR9_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR9_4` reader - GR9_4 analog switch control"]
pub type GR9_4_R = crate::BitReader<bool>;
#[doc = "Field `GR9_4` writer - GR9_4 analog switch control"]
pub type GR9_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR2_3` reader - GR2_3 analog switch control"]
pub type GR2_3_R = crate::BitReader<bool>;
#[doc = "Field `GR2_3` writer - GR2_3 analog switch control"]
pub type GR2_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR2_4` reader - GR2_4 analog switch control"]
pub type GR2_4_R = crate::BitReader<bool>;
#[doc = "Field `GR2_4` writer - GR2_4 analog switch control"]
pub type GR2_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR2_5` reader - GR2_5 analog switch control"]
pub type GR2_5_R = crate::BitReader<bool>;
#[doc = "Field `GR2_5` writer - GR2_5 analog switch control"]
pub type GR2_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR7_5` reader - GR7_5 analog switch control"]
pub type GR7_5_R = crate::BitReader<bool>;
#[doc = "Field `GR7_5` writer - GR7_5 analog switch control"]
pub type GR7_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR7_6` reader - GR7_6 analog switch control"]
pub type GR7_6_R = crate::BitReader<bool>;
#[doc = "Field `GR7_6` writer - GR7_6 analog switch control"]
pub type GR7_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR7_7` reader - GR7_7 analog switch control"]
pub type GR7_7_R = crate::BitReader<bool>;
#[doc = "Field `GR7_7` writer - GR7_7 analog switch control"]
pub type GR7_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR6_3` reader - GR6_3 analog switch control"]
pub type GR6_3_R = crate::BitReader<bool>;
#[doc = "Field `GR6_3` writer - GR6_3 analog switch control"]
pub type GR6_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR6_4` reader - GR6_4 analog switch control"]
pub type GR6_4_R = crate::BitReader<bool>;
#[doc = "Field `GR6_4` writer - GR6_4 analog switch control"]
pub type GR6_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
#[doc = "Field `GR5_4` reader - GR5_4 analog switch control"]
pub type GR5_4_R = crate::BitReader<bool>;
#[doc = "Field `GR5_4` writer - GR5_4 analog switch control"]
pub type GR5_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GR10_1 analog switch control"]
    #[inline(always)]
    pub fn gr10_1(&self) -> GR10_1_R {
        GR10_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GR10_2 analog switch control"]
    #[inline(always)]
    pub fn gr10_2(&self) -> GR10_2_R {
        GR10_2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GR10_3 analog switch control"]
    #[inline(always)]
    pub fn gr10_3(&self) -> GR10_3_R {
        GR10_3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GR10_4 analog switch control"]
    #[inline(always)]
    pub fn gr10_4(&self) -> GR10_4_R {
        GR10_4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GR6_1 analog switch control"]
    #[inline(always)]
    pub fn gr6_1(&self) -> GR6_1_R {
        GR6_1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GR6_2 analog switch control"]
    #[inline(always)]
    pub fn gr6_2(&self) -> GR6_2_R {
        GR6_2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GR5_1 analog switch control"]
    #[inline(always)]
    pub fn gr5_1(&self) -> GR5_1_R {
        GR5_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GR5_2 analog switch control"]
    #[inline(always)]
    pub fn gr5_2(&self) -> GR5_2_R {
        GR5_2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GR5_3 analog switch control"]
    #[inline(always)]
    pub fn gr5_3(&self) -> GR5_3_R {
        GR5_3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GR4_1 analog switch control"]
    #[inline(always)]
    pub fn gr4_1(&self) -> GR4_1_R {
        GR4_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GR4_2 analog switch control"]
    #[inline(always)]
    pub fn gr4_2(&self) -> GR4_2_R {
        GR4_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GR4_3 analog switch control"]
    #[inline(always)]
    pub fn gr4_3(&self) -> GR4_3_R {
        GR4_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GR3_3 analog switch control"]
    #[inline(always)]
    pub fn gr3_3(&self) -> GR3_3_R {
        GR3_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GR3_4 analog switch control"]
    #[inline(always)]
    pub fn gr3_4(&self) -> GR3_4_R {
        GR3_4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GR3_5 analog switch control"]
    #[inline(always)]
    pub fn gr3_5(&self) -> GR3_5_R {
        GR3_5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GR9_3 analog switch control"]
    #[inline(always)]
    pub fn gr9_3(&self) -> GR9_3_R {
        GR9_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GR9_4 analog switch control"]
    #[inline(always)]
    pub fn gr9_4(&self) -> GR9_4_R {
        GR9_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GR2_3 analog switch control"]
    #[inline(always)]
    pub fn gr2_3(&self) -> GR2_3_R {
        GR2_3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GR2_4 analog switch control"]
    #[inline(always)]
    pub fn gr2_4(&self) -> GR2_4_R {
        GR2_4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GR2_5 analog switch control"]
    #[inline(always)]
    pub fn gr2_5(&self) -> GR2_5_R {
        GR2_5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GR7_5 analog switch control"]
    #[inline(always)]
    pub fn gr7_5(&self) -> GR7_5_R {
        GR7_5_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GR7_6 analog switch control"]
    #[inline(always)]
    pub fn gr7_6(&self) -> GR7_6_R {
        GR7_6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GR7_7 analog switch control"]
    #[inline(always)]
    pub fn gr7_7(&self) -> GR7_7_R {
        GR7_7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GR6_3 analog switch control"]
    #[inline(always)]
    pub fn gr6_3(&self) -> GR6_3_R {
        GR6_3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GR6_4 analog switch control"]
    #[inline(always)]
    pub fn gr6_4(&self) -> GR6_4_R {
        GR6_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GR5_4 analog switch control"]
    #[inline(always)]
    pub fn gr5_4(&self) -> GR5_4_R {
        GR5_4_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GR10_1 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr10_1(&mut self) -> GR10_1_W<0> {
        GR10_1_W::new(self)
    }
    #[doc = "Bit 1 - GR10_2 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr10_2(&mut self) -> GR10_2_W<1> {
        GR10_2_W::new(self)
    }
    #[doc = "Bit 2 - GR10_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr10_3(&mut self) -> GR10_3_W<2> {
        GR10_3_W::new(self)
    }
    #[doc = "Bit 3 - GR10_4 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr10_4(&mut self) -> GR10_4_W<3> {
        GR10_4_W::new(self)
    }
    #[doc = "Bit 4 - GR6_1 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr6_1(&mut self) -> GR6_1_W<4> {
        GR6_1_W::new(self)
    }
    #[doc = "Bit 5 - GR6_2 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr6_2(&mut self) -> GR6_2_W<5> {
        GR6_2_W::new(self)
    }
    #[doc = "Bit 6 - GR5_1 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr5_1(&mut self) -> GR5_1_W<6> {
        GR5_1_W::new(self)
    }
    #[doc = "Bit 7 - GR5_2 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr5_2(&mut self) -> GR5_2_W<7> {
        GR5_2_W::new(self)
    }
    #[doc = "Bit 8 - GR5_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr5_3(&mut self) -> GR5_3_W<8> {
        GR5_3_W::new(self)
    }
    #[doc = "Bit 9 - GR4_1 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr4_1(&mut self) -> GR4_1_W<9> {
        GR4_1_W::new(self)
    }
    #[doc = "Bit 10 - GR4_2 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr4_2(&mut self) -> GR4_2_W<10> {
        GR4_2_W::new(self)
    }
    #[doc = "Bit 11 - GR4_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr4_3(&mut self) -> GR4_3_W<11> {
        GR4_3_W::new(self)
    }
    #[doc = "Bit 16 - GR3_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr3_3(&mut self) -> GR3_3_W<16> {
        GR3_3_W::new(self)
    }
    #[doc = "Bit 17 - GR3_4 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr3_4(&mut self) -> GR3_4_W<17> {
        GR3_4_W::new(self)
    }
    #[doc = "Bit 18 - GR3_5 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr3_5(&mut self) -> GR3_5_W<18> {
        GR3_5_W::new(self)
    }
    #[doc = "Bit 19 - GR9_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr9_3(&mut self) -> GR9_3_W<19> {
        GR9_3_W::new(self)
    }
    #[doc = "Bit 20 - GR9_4 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr9_4(&mut self) -> GR9_4_W<20> {
        GR9_4_W::new(self)
    }
    #[doc = "Bit 21 - GR2_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr2_3(&mut self) -> GR2_3_W<21> {
        GR2_3_W::new(self)
    }
    #[doc = "Bit 22 - GR2_4 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr2_4(&mut self) -> GR2_4_W<22> {
        GR2_4_W::new(self)
    }
    #[doc = "Bit 23 - GR2_5 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr2_5(&mut self) -> GR2_5_W<23> {
        GR2_5_W::new(self)
    }
    #[doc = "Bit 24 - GR7_5 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr7_5(&mut self) -> GR7_5_W<24> {
        GR7_5_W::new(self)
    }
    #[doc = "Bit 25 - GR7_6 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr7_6(&mut self) -> GR7_6_W<25> {
        GR7_6_W::new(self)
    }
    #[doc = "Bit 26 - GR7_7 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr7_7(&mut self) -> GR7_7_W<26> {
        GR7_7_W::new(self)
    }
    #[doc = "Bit 27 - GR6_3 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr6_3(&mut self) -> GR6_3_W<27> {
        GR6_3_W::new(self)
    }
    #[doc = "Bit 28 - GR6_4 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr6_4(&mut self) -> GR6_4_W<28> {
        GR6_4_W::new(self)
    }
    #[doc = "Bit 29 - GR5_4 analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn gr5_4(&mut self) -> GR5_4_W<29> {
        GR5_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI analog switches control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr2](index.html) module"]
pub struct ASCR2_SPEC;
impl crate::RegisterSpec for ASCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ascr2::R](R) reader structure"]
impl crate::Readable for ASCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ascr2::W](W) writer structure"]
impl crate::Writable for ASCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASCR2 to value 0"]
impl crate::Resettable for ASCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

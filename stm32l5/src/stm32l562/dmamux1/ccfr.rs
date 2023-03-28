#[doc = "Register `CCFR` reader"]
pub struct R(crate::R<CCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFR` writer"]
pub struct W(crate::W<CCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFR_SPEC>;
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
impl From<crate::W<CCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSOF0` reader - Synchronization Clear Overrun Flag 0"]
pub type CSOF0_R = crate::BitReader<bool>;
#[doc = "Field `CSOF0` writer - Synchronization Clear Overrun Flag 0"]
pub type CSOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF1` reader - Synchronization Clear Overrun Flag 1"]
pub type CSOF1_R = crate::BitReader<bool>;
#[doc = "Field `CSOF1` writer - Synchronization Clear Overrun Flag 1"]
pub type CSOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF2` reader - Synchronization Clear Overrun Flag 2"]
pub type CSOF2_R = crate::BitReader<bool>;
#[doc = "Field `CSOF2` writer - Synchronization Clear Overrun Flag 2"]
pub type CSOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF3` reader - Synchronization Clear Overrun Flag 3"]
pub type CSOF3_R = crate::BitReader<bool>;
#[doc = "Field `CSOF3` writer - Synchronization Clear Overrun Flag 3"]
pub type CSOF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF4` reader - Synchronization Clear Overrun Flag 4"]
pub type CSOF4_R = crate::BitReader<bool>;
#[doc = "Field `CSOF4` writer - Synchronization Clear Overrun Flag 4"]
pub type CSOF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF5` reader - Synchronization Clear Overrun Flag 5"]
pub type CSOF5_R = crate::BitReader<bool>;
#[doc = "Field `CSOF5` writer - Synchronization Clear Overrun Flag 5"]
pub type CSOF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF6` reader - Synchronization Clear Overrun Flag 6"]
pub type CSOF6_R = crate::BitReader<bool>;
#[doc = "Field `CSOF6` writer - Synchronization Clear Overrun Flag 6"]
pub type CSOF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF7` reader - Synchronization Clear Overrun Flag 7"]
pub type CSOF7_R = crate::BitReader<bool>;
#[doc = "Field `CSOF7` writer - Synchronization Clear Overrun Flag 7"]
pub type CSOF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF8` reader - Synchronization Clear Overrun Flag 8"]
pub type CSOF8_R = crate::BitReader<bool>;
#[doc = "Field `CSOF8` writer - Synchronization Clear Overrun Flag 8"]
pub type CSOF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF9` reader - Synchronization Clear Overrun Flag 9"]
pub type CSOF9_R = crate::BitReader<bool>;
#[doc = "Field `CSOF9` writer - Synchronization Clear Overrun Flag 9"]
pub type CSOF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF10` reader - Synchronization Clear Overrun Flag 10"]
pub type CSOF10_R = crate::BitReader<bool>;
#[doc = "Field `CSOF10` writer - Synchronization Clear Overrun Flag 10"]
pub type CSOF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF11` reader - Synchronization Clear Overrun Flag 11"]
pub type CSOF11_R = crate::BitReader<bool>;
#[doc = "Field `CSOF11` writer - Synchronization Clear Overrun Flag 11"]
pub type CSOF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF12` reader - Synchronization Clear Overrun Flag 12"]
pub type CSOF12_R = crate::BitReader<bool>;
#[doc = "Field `CSOF12` writer - Synchronization Clear Overrun Flag 12"]
pub type CSOF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF13` reader - Synchronization Clear Overrun Flag 13"]
pub type CSOF13_R = crate::BitReader<bool>;
#[doc = "Field `CSOF13` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF14` reader - Synchronization Clear Overrun Flag 13"]
pub type CSOF14_R = crate::BitReader<bool>;
#[doc = "Field `CSOF14` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
#[doc = "Field `CSOF15` reader - Synchronization Clear Overrun Flag 13"]
pub type CSOF15_R = crate::BitReader<bool>;
#[doc = "Field `CSOF15` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    pub fn csof4(&self) -> CSOF4_R {
        CSOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    pub fn csof5(&self) -> CSOF5_R {
        CSOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    pub fn csof6(&self) -> CSOF6_R {
        CSOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    pub fn csof7(&self) -> CSOF7_R {
        CSOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    pub fn csof8(&self) -> CSOF8_R {
        CSOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    pub fn csof9(&self) -> CSOF9_R {
        CSOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    pub fn csof10(&self) -> CSOF10_R {
        CSOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    pub fn csof11(&self) -> CSOF11_R {
        CSOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    pub fn csof12(&self) -> CSOF12_R {
        CSOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof13(&self) -> CSOF13_R {
        CSOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof14(&self) -> CSOF14_R {
        CSOF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof15(&self) -> CSOF15_R {
        CSOF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<0> {
        CSOF0_W::new(self)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<1> {
        CSOF1_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<2> {
        CSOF2_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<3> {
        CSOF3_W::new(self)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<4> {
        CSOF4_W::new(self)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<5> {
        CSOF5_W::new(self)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<6> {
        CSOF6_W::new(self)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<7> {
        CSOF7_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<8> {
        CSOF8_W::new(self)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<9> {
        CSOF9_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<10> {
        CSOF10_W::new(self)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<11> {
        CSOF11_W::new(self)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<12> {
        CSOF12_W::new(self)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<13> {
        CSOF13_W::new(self)
    }
    #[doc = "Bit 14 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof14(&mut self) -> CSOF14_W<14> {
        CSOF14_W::new(self)
    }
    #[doc = "Bit 15 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof15(&mut self) -> CSOF15_W<15> {
        CSOF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Clear Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfr](index.html) module"]
pub struct CCFR_SPEC;
impl crate::RegisterSpec for CCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfr::R](R) reader structure"]
impl crate::Readable for CCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfr::W](W) writer structure"]
impl crate::Writable for CCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFR to value 0"]
impl crate::Resettable for CCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

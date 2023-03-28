#[doc = "Register `PR1` reader"]
pub struct R(crate::R<PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR1` writer"]
pub struct W(crate::W<PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR1_SPEC>;
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
impl From<crate::W<PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIF0` reader - Configurable event inputs Pending bit"]
pub type PIF0_R = crate::BitReader<bool>;
#[doc = "Field `PIF0` writer - Configurable event inputs Pending bit"]
pub type PIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF1` reader - Configurable event inputs Pending bit"]
pub type PIF1_R = crate::BitReader<bool>;
#[doc = "Field `PIF1` writer - Configurable event inputs Pending bit"]
pub type PIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF2` reader - Configurable event inputs Pending bit"]
pub type PIF2_R = crate::BitReader<bool>;
#[doc = "Field `PIF2` writer - Configurable event inputs Pending bit"]
pub type PIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF3` reader - Configurable event inputs Pending bit"]
pub type PIF3_R = crate::BitReader<bool>;
#[doc = "Field `PIF3` writer - Configurable event inputs Pending bit"]
pub type PIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF4` reader - Configurable event inputs Pending bit"]
pub type PIF4_R = crate::BitReader<bool>;
#[doc = "Field `PIF4` writer - Configurable event inputs Pending bit"]
pub type PIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF5` reader - Configurable event inputs Pending bit"]
pub type PIF5_R = crate::BitReader<bool>;
#[doc = "Field `PIF5` writer - Configurable event inputs Pending bit"]
pub type PIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF6` reader - Configurable event inputs Pending bit"]
pub type PIF6_R = crate::BitReader<bool>;
#[doc = "Field `PIF6` writer - Configurable event inputs Pending bit"]
pub type PIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF7` reader - Configurable event inputs Pending bit"]
pub type PIF7_R = crate::BitReader<bool>;
#[doc = "Field `PIF7` writer - Configurable event inputs Pending bit"]
pub type PIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF8` reader - Configurable event inputs Pending bit"]
pub type PIF8_R = crate::BitReader<bool>;
#[doc = "Field `PIF8` writer - Configurable event inputs Pending bit"]
pub type PIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF9` reader - Configurable event inputs Pending bit"]
pub type PIF9_R = crate::BitReader<bool>;
#[doc = "Field `PIF9` writer - Configurable event inputs Pending bit"]
pub type PIF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF10` reader - Configurable event inputs Pending bit"]
pub type PIF10_R = crate::BitReader<bool>;
#[doc = "Field `PIF10` writer - Configurable event inputs Pending bit"]
pub type PIF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF11` reader - Configurable event inputs Pending bit"]
pub type PIF11_R = crate::BitReader<bool>;
#[doc = "Field `PIF11` writer - Configurable event inputs Pending bit"]
pub type PIF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF12` reader - Configurable event inputs Pending bit"]
pub type PIF12_R = crate::BitReader<bool>;
#[doc = "Field `PIF12` writer - Configurable event inputs Pending bit"]
pub type PIF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF13` reader - Configurable event inputs Pending bit"]
pub type PIF13_R = crate::BitReader<bool>;
#[doc = "Field `PIF13` writer - Configurable event inputs Pending bit"]
pub type PIF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF14` reader - Configurable event inputs Pending bit"]
pub type PIF14_R = crate::BitReader<bool>;
#[doc = "Field `PIF14` writer - Configurable event inputs Pending bit"]
pub type PIF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF15` reader - Configurable event inputs Pending bit"]
pub type PIF15_R = crate::BitReader<bool>;
#[doc = "Field `PIF15` writer - Configurable event inputs Pending bit"]
pub type PIF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF16` reader - Configurable event inputs Pending bit"]
pub type PIF16_R = crate::BitReader<bool>;
#[doc = "Field `PIF16` writer - Configurable event inputs Pending bit"]
pub type PIF16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF17` reader - Configurable event inputs Pending bit"]
pub type PIF17_R = crate::BitReader<bool>;
#[doc = "Field `PIF17` writer - Configurable event inputs Pending bit"]
pub type PIF17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF18` reader - Configurable event inputs Pending bit"]
pub type PIF18_R = crate::BitReader<bool>;
#[doc = "Field `PIF18` writer - Configurable event inputs Pending bit"]
pub type PIF18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF19` reader - Configurable event inputs Pending bit"]
pub type PIF19_R = crate::BitReader<bool>;
#[doc = "Field `PIF19` writer - Configurable event inputs Pending bit"]
pub type PIF19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF20` reader - Configurable event inputs Pending bit"]
pub type PIF20_R = crate::BitReader<bool>;
#[doc = "Field `PIF20` writer - Configurable event inputs Pending bit"]
pub type PIF20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF21` reader - Configurable event inputs Pending bit"]
pub type PIF21_R = crate::BitReader<bool>;
#[doc = "Field `PIF21` writer - Configurable event inputs Pending bit"]
pub type PIF21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
#[doc = "Field `PIF31` reader - Configurable event inputs Pending bit"]
pub type PIF31_R = crate::BitReader<bool>;
#[doc = "Field `PIF31` writer - Configurable event inputs Pending bit"]
pub type PIF31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif17(&self) -> PIF17_R {
        PIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif18(&self) -> PIF18_R {
        PIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif19(&self) -> PIF19_R {
        PIF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif20(&self) -> PIF20_R {
        PIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif31(&self) -> PIF31_R {
        PIF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> PIF0_W<0> {
        PIF0_W::new(self)
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> PIF1_W<1> {
        PIF1_W::new(self)
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> PIF2_W<2> {
        PIF2_W::new(self)
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> PIF3_W<3> {
        PIF3_W::new(self)
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> PIF4_W<4> {
        PIF4_W::new(self)
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> PIF5_W<5> {
        PIF5_W::new(self)
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> PIF6_W<6> {
        PIF6_W::new(self)
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> PIF7_W<7> {
        PIF7_W::new(self)
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif8(&mut self) -> PIF8_W<8> {
        PIF8_W::new(self)
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif9(&mut self) -> PIF9_W<9> {
        PIF9_W::new(self)
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif10(&mut self) -> PIF10_W<10> {
        PIF10_W::new(self)
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> PIF11_W<11> {
        PIF11_W::new(self)
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif12(&mut self) -> PIF12_W<12> {
        PIF12_W::new(self)
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif13(&mut self) -> PIF13_W<13> {
        PIF13_W::new(self)
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif14(&mut self) -> PIF14_W<14> {
        PIF14_W::new(self)
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif15(&mut self) -> PIF15_W<15> {
        PIF15_W::new(self)
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif16(&mut self) -> PIF16_W<16> {
        PIF16_W::new(self)
    }
    #[doc = "Bit 17 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif17(&mut self) -> PIF17_W<17> {
        PIF17_W::new(self)
    }
    #[doc = "Bit 18 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif18(&mut self) -> PIF18_W<18> {
        PIF18_W::new(self)
    }
    #[doc = "Bit 19 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif19(&mut self) -> PIF19_W<19> {
        PIF19_W::new(self)
    }
    #[doc = "Bit 20 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif20(&mut self) -> PIF20_W<20> {
        PIF20_W::new(self)
    }
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif21(&mut self) -> PIF21_W<21> {
        PIF21_W::new(self)
    }
    #[doc = "Bit 31 - Configurable event inputs Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pif31(&mut self) -> PIF31_W<31> {
        PIF31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1](index.html) module"]
pub struct PR1_SPEC;
impl crate::RegisterSpec for PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr1::R](R) reader structure"]
impl crate::Readable for PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr1::W](W) writer structure"]
impl crate::Writable for PR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR1 to value 0"]
impl crate::Resettable for PR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

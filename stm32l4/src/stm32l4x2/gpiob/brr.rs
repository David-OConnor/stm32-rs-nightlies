#[doc = "Register `BRR` reader"]
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR0` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub type BR0_R = crate::BitReader<BR0W_A>;
#[doc = "These bits are write-only. A read to these bits returns the value 0x0000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W_A {
    #[doc = "0: No action on the corresponding ODx bit"]
    NoAction = 0,
    #[doc = "1: Reset the corresponding ODx bit"]
    Reset = 1,
}
impl From<BR0W_A> for bool {
    #[inline(always)]
    fn from(variant: BR0W_A) -> Self {
        variant as u8 != 0
    }
}
impl BR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BR0W_A {
        match self.bits {
            false => BR0W_A::NoAction,
            true => BR0W_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoAction`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BR0W_A::NoAction
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BR0W_A::Reset
    }
}
#[doc = "Field `BR0` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, BR0W_A, O>;
impl<'a, const O: u8> BR0_W<'a, O> {
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W_A::NoAction)
    }
    #[doc = "Reset the corresponding ODx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W_A::Reset)
    }
}
#[doc = "Field `BR1` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR1_R;
#[doc = "Field `BR2` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR2_R;
#[doc = "Field `BR3` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR3_R;
#[doc = "Field `BR4` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR4_R;
#[doc = "Field `BR5` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR5_R;
#[doc = "Field `BR6` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR6_R;
#[doc = "Field `BR7` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR7_R;
#[doc = "Field `BR8` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR8_R;
#[doc = "Field `BR9` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR9_R;
#[doc = "Field `BR10` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR10_R;
#[doc = "Field `BR11` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR11_R;
#[doc = "Field `BR12` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR12_R;
#[doc = "Field `BR13` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR13_R;
#[doc = "Field `BR14` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR14_R;
#[doc = "Field `BR15` reader - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_R as BR15_R;
#[doc = "Field `BR1` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR1_W;
#[doc = "Field `BR2` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR2_W;
#[doc = "Field `BR3` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR3_W;
#[doc = "Field `BR4` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR4_W;
#[doc = "Field `BR5` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR5_W;
#[doc = "Field `BR6` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR6_W;
#[doc = "Field `BR7` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR7_W;
#[doc = "Field `BR8` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR8_W;
#[doc = "Field `BR9` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR9_W;
#[doc = "Field `BR10` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR10_W;
#[doc = "Field `BR11` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR11_W;
#[doc = "Field `BR12` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR12_W;
#[doc = "Field `BR13` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR13_W;
#[doc = "Field `BR14` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR14_W;
#[doc = "Field `BR15` writer - These bits are write-only. A read to these bits returns the value 0x0000"]
pub use BR0_W as BR15_W;
impl R {
    #[doc = "Bit 0 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br0(&self) -> BR0_R {
        BR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br1(&self) -> BR1_R {
        BR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br2(&self) -> BR2_R {
        BR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br4(&self) -> BR4_R {
        BR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br5(&self) -> BR5_R {
        BR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br6(&self) -> BR6_R {
        BR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br7(&self) -> BR7_R {
        BR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br8(&self) -> BR8_R {
        BR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br9(&self) -> BR9_R {
        BR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br10(&self) -> BR10_R {
        BR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br11(&self) -> BR11_R {
        BR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br12(&self) -> BR12_R {
        BR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br13(&self) -> BR13_R {
        BR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br14(&self) -> BR14_R {
        BR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    pub fn br15(&self) -> BR15_R {
        BR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - These bits are write-only. A read to these bits returns the value 0x0000"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<15> {
        BR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brr::R](R) reader structure"]
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

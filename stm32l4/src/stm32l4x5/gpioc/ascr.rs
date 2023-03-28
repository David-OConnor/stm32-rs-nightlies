#[doc = "Register `ASCR` reader"]
pub struct R(crate::R<ASCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCR` writer"]
pub struct W(crate::W<ASCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCR_SPEC>;
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
impl From<crate::W<ASCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASC0` reader - These bits are written by software to configure the analog connection of the IOs."]
pub type ASC0_R = crate::BitReader<ASC0W_A>;
#[doc = "These bits are written by software to configure the analog connection of the IOs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASC0W_A {
    #[doc = "0: Disconnect analog switch to the ADC input"]
    NoAction = 0,
    #[doc = "1: Connect analog switch to the ADC input"]
    Reset = 1,
}
impl From<ASC0W_A> for bool {
    #[inline(always)]
    fn from(variant: ASC0W_A) -> Self {
        variant as u8 != 0
    }
}
impl ASC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASC0W_A {
        match self.bits {
            false => ASC0W_A::NoAction,
            true => ASC0W_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoAction`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ASC0W_A::NoAction
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ASC0W_A::Reset
    }
}
#[doc = "Field `ASC0` writer - These bits are written by software to configure the analog connection of the IOs."]
pub type ASC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, ASC0W_A, O>;
impl<'a, const O: u8> ASC0_W<'a, O> {
    #[doc = "Disconnect analog switch to the ADC input"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(ASC0W_A::NoAction)
    }
    #[doc = "Connect analog switch to the ADC input"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ASC0W_A::Reset)
    }
}
#[doc = "Field `ASC1` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC1_R;
#[doc = "Field `ASC2` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC2_R;
#[doc = "Field `ASC3` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC3_R;
#[doc = "Field `ASC4` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC4_R;
#[doc = "Field `ASC5` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC5_R;
#[doc = "Field `ASC6` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC6_R;
#[doc = "Field `ASC7` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC7_R;
#[doc = "Field `ASC8` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC8_R;
#[doc = "Field `ASC9` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC9_R;
#[doc = "Field `ASC10` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC10_R;
#[doc = "Field `ASC11` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC11_R;
#[doc = "Field `ASC12` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC12_R;
#[doc = "Field `ASC13` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC13_R;
#[doc = "Field `ASC14` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC14_R;
#[doc = "Field `ASC15` reader - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_R as ASC15_R;
#[doc = "Field `ASC1` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC1_W;
#[doc = "Field `ASC2` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC2_W;
#[doc = "Field `ASC3` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC3_W;
#[doc = "Field `ASC4` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC4_W;
#[doc = "Field `ASC5` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC5_W;
#[doc = "Field `ASC6` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC6_W;
#[doc = "Field `ASC7` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC7_W;
#[doc = "Field `ASC8` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC8_W;
#[doc = "Field `ASC9` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC9_W;
#[doc = "Field `ASC10` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC10_W;
#[doc = "Field `ASC11` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC11_W;
#[doc = "Field `ASC12` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC12_W;
#[doc = "Field `ASC13` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC13_W;
#[doc = "Field `ASC14` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC14_W;
#[doc = "Field `ASC15` writer - These bits are written by software to configure the analog connection of the IOs."]
pub use ASC0_W as ASC15_W;
impl R {
    #[doc = "Bit 0 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc0(&mut self) -> ASC0_W<0> {
        ASC0_W::new(self)
    }
    #[doc = "Bit 1 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc1(&mut self) -> ASC1_W<1> {
        ASC1_W::new(self)
    }
    #[doc = "Bit 2 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc2(&mut self) -> ASC2_W<2> {
        ASC2_W::new(self)
    }
    #[doc = "Bit 3 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc3(&mut self) -> ASC3_W<3> {
        ASC3_W::new(self)
    }
    #[doc = "Bit 4 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc4(&mut self) -> ASC4_W<4> {
        ASC4_W::new(self)
    }
    #[doc = "Bit 5 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc5(&mut self) -> ASC5_W<5> {
        ASC5_W::new(self)
    }
    #[doc = "Bit 6 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc6(&mut self) -> ASC6_W<6> {
        ASC6_W::new(self)
    }
    #[doc = "Bit 7 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc7(&mut self) -> ASC7_W<7> {
        ASC7_W::new(self)
    }
    #[doc = "Bit 8 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc8(&mut self) -> ASC8_W<8> {
        ASC8_W::new(self)
    }
    #[doc = "Bit 9 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc9(&mut self) -> ASC9_W<9> {
        ASC9_W::new(self)
    }
    #[doc = "Bit 10 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc10(&mut self) -> ASC10_W<10> {
        ASC10_W::new(self)
    }
    #[doc = "Bit 11 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc11(&mut self) -> ASC11_W<11> {
        ASC11_W::new(self)
    }
    #[doc = "Bit 12 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc12(&mut self) -> ASC12_W<12> {
        ASC12_W::new(self)
    }
    #[doc = "Bit 13 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc13(&mut self) -> ASC13_W<13> {
        ASC13_W::new(self)
    }
    #[doc = "Bit 14 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc14(&mut self) -> ASC14_W<14> {
        ASC14_W::new(self)
    }
    #[doc = "Bit 15 - These bits are written by software to configure the analog connection of the IOs."]
    #[inline(always)]
    #[must_use]
    pub fn asc15(&mut self) -> ASC15_W<15> {
        ASC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port analog switch control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr](index.html) module"]
pub struct ASCR_SPEC;
impl crate::RegisterSpec for ASCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ascr::R](R) reader structure"]
impl crate::Readable for ASCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ascr::W](W) writer structure"]
impl crate::Writable for ASCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASCR to value 0"]
impl crate::Resettable for ASCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

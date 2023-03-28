#[doc = "Register `CRH` reader"]
pub struct R(crate::R<CRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRH` writer"]
pub struct W(crate::W<CRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRH_SPEC>;
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
impl From<crate::W<CRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE8` reader - Port n.8 mode bits"]
pub type MODE8_R = crate::FieldReader<u8, MODE8_A>;
#[doc = "Port n.8 mode bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE8_A {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: Output mode 10 MHz"]
    Output = 1,
    #[doc = "2: Output mode 2 MHz"]
    Output2 = 2,
    #[doc = "3: Output mode 50 MHz"]
    Output50 = 3,
}
impl From<MODE8_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as _
    }
}
impl MODE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            0 => MODE8_A::Input,
            1 => MODE8_A::Output,
            2 => MODE8_A::Output2,
            3 => MODE8_A::Output50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Input`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE8_A::Input
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE8_A::Output
    }
    #[doc = "Checks if the value of the field is `Output2`"]
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        *self == MODE8_A::Output2
    }
    #[doc = "Checks if the value of the field is `Output50`"]
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        *self == MODE8_A::Output50
    }
}
#[doc = "Field `MODE8` writer - Port n.8 mode bits"]
pub type MODE8_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CRH_SPEC, u8, MODE8_A, 2, O>;
impl<'a, const O: u8> MODE8_W<'a, O> {
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE8_A::Input)
    }
    #[doc = "Output mode 10 MHz"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE8_A::Output)
    }
    #[doc = "Output mode 2 MHz"]
    #[inline(always)]
    pub fn output2(self) -> &'a mut W {
        self.variant(MODE8_A::Output2)
    }
    #[doc = "Output mode 50 MHz"]
    #[inline(always)]
    pub fn output50(self) -> &'a mut W {
        self.variant(MODE8_A::Output50)
    }
}
#[doc = "Field `CNF8` reader - Port n.8 configuration bits"]
pub type CNF8_R = crate::FieldReader<u8, CNF8_A>;
#[doc = "Port n.8 configuration bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNF8_A {
    #[doc = "0: Analog mode / Push-Pull mode"]
    PushPull = 0,
    #[doc = "1: Floating input (reset state) / Open Drain-Mode"]
    OpenDrain = 1,
    #[doc = "2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    AltPushPull = 2,
    #[doc = "3: Alternate Function Open-Drain Mode"]
    AltOpenDrain = 3,
}
impl From<CNF8_A> for u8 {
    #[inline(always)]
    fn from(variant: CNF8_A) -> Self {
        variant as _
    }
}
impl CNF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNF8_A {
        match self.bits {
            0 => CNF8_A::PushPull,
            1 => CNF8_A::OpenDrain,
            2 => CNF8_A::AltPushPull,
            3 => CNF8_A::AltOpenDrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PushPull`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == CNF8_A::PushPull
    }
    #[doc = "Checks if the value of the field is `OpenDrain`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == CNF8_A::OpenDrain
    }
    #[doc = "Checks if the value of the field is `AltPushPull`"]
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        *self == CNF8_A::AltPushPull
    }
    #[doc = "Checks if the value of the field is `AltOpenDrain`"]
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        *self == CNF8_A::AltOpenDrain
    }
}
#[doc = "Field `CNF8` writer - Port n.8 configuration bits"]
pub type CNF8_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CRH_SPEC, u8, CNF8_A, 2, O>;
impl<'a, const O: u8> CNF8_W<'a, O> {
    #[doc = "Analog mode / Push-Pull mode"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(CNF8_A::PushPull)
    }
    #[doc = "Floating input (reset state) / Open Drain-Mode"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(CNF8_A::OpenDrain)
    }
    #[doc = "Input with pull-up/pull-down / Alternate Function Push-Pull Mode"]
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut W {
        self.variant(CNF8_A::AltPushPull)
    }
    #[doc = "Alternate Function Open-Drain Mode"]
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut W {
        self.variant(CNF8_A::AltOpenDrain)
    }
}
#[doc = "Field `CNF9` reader - Port n.9 configuration bits"]
pub use CNF8_R as CNF9_R;
#[doc = "Field `CNF10` reader - Port n.10 configuration bits"]
pub use CNF8_R as CNF10_R;
#[doc = "Field `CNF11` reader - Port n.11 configuration bits"]
pub use CNF8_R as CNF11_R;
#[doc = "Field `CNF12` reader - Port n.12 configuration bits"]
pub use CNF8_R as CNF12_R;
#[doc = "Field `CNF13` reader - Port n.13 configuration bits"]
pub use CNF8_R as CNF13_R;
#[doc = "Field `CNF14` reader - Port n.14 configuration bits"]
pub use CNF8_R as CNF14_R;
#[doc = "Field `CNF15` reader - Port n.15 configuration bits"]
pub use CNF8_R as CNF15_R;
#[doc = "Field `CNF9` writer - Port n.9 configuration bits"]
pub use CNF8_W as CNF9_W;
#[doc = "Field `CNF10` writer - Port n.10 configuration bits"]
pub use CNF8_W as CNF10_W;
#[doc = "Field `CNF11` writer - Port n.11 configuration bits"]
pub use CNF8_W as CNF11_W;
#[doc = "Field `CNF12` writer - Port n.12 configuration bits"]
pub use CNF8_W as CNF12_W;
#[doc = "Field `CNF13` writer - Port n.13 configuration bits"]
pub use CNF8_W as CNF13_W;
#[doc = "Field `CNF14` writer - Port n.14 configuration bits"]
pub use CNF8_W as CNF14_W;
#[doc = "Field `CNF15` writer - Port n.15 configuration bits"]
pub use CNF8_W as CNF15_W;
#[doc = "Field `MODE9` reader - Port n.9 mode bits"]
pub use MODE8_R as MODE9_R;
#[doc = "Field `MODE10` reader - Port n.10 mode bits"]
pub use MODE8_R as MODE10_R;
#[doc = "Field `MODE11` reader - Port n.11 mode bits"]
pub use MODE8_R as MODE11_R;
#[doc = "Field `MODE12` reader - Port n.12 mode bits"]
pub use MODE8_R as MODE12_R;
#[doc = "Field `MODE13` reader - Port n.13 mode bits"]
pub use MODE8_R as MODE13_R;
#[doc = "Field `MODE14` reader - Port n.14 mode bits"]
pub use MODE8_R as MODE14_R;
#[doc = "Field `MODE15` reader - Port n.15 mode bits"]
pub use MODE8_R as MODE15_R;
#[doc = "Field `MODE9` writer - Port n.9 mode bits"]
pub use MODE8_W as MODE9_W;
#[doc = "Field `MODE10` writer - Port n.10 mode bits"]
pub use MODE8_W as MODE10_W;
#[doc = "Field `MODE11` writer - Port n.11 mode bits"]
pub use MODE8_W as MODE11_W;
#[doc = "Field `MODE12` writer - Port n.12 mode bits"]
pub use MODE8_W as MODE12_W;
#[doc = "Field `MODE13` writer - Port n.13 mode bits"]
pub use MODE8_W as MODE13_W;
#[doc = "Field `MODE14` writer - Port n.14 mode bits"]
pub use MODE8_W as MODE14_W;
#[doc = "Field `MODE15` writer - Port n.15 mode bits"]
pub use MODE8_W as MODE15_W;
impl R {
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&self) -> CNF8_R {
        CNF8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&self) -> CNF9_R {
        CNF9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&self) -> CNF10_R {
        CNF10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&self) -> CNF11_R {
        CNF11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&self) -> CNF12_R {
        CNF12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&self) -> CNF13_R {
        CNF13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&self) -> CNF14_R {
        CNF14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&self) -> CNF15_R {
        CNF15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> MODE8_W<0> {
        MODE8_W::new(self)
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf8(&mut self) -> CNF8_W<2> {
        CNF8_W::new(self)
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<4> {
        MODE9_W::new(self)
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf9(&mut self) -> CNF9_W<6> {
        CNF9_W::new(self)
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> MODE10_W<8> {
        MODE10_W::new(self)
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf10(&mut self) -> CNF10_W<10> {
        CNF10_W::new(self)
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> MODE11_W<12> {
        MODE11_W::new(self)
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf11(&mut self) -> CNF11_W<14> {
        CNF11_W::new(self)
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> MODE12_W<16> {
        MODE12_W::new(self)
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf12(&mut self) -> CNF12_W<18> {
        CNF12_W::new(self)
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> MODE13_W<20> {
        MODE13_W::new(self)
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf13(&mut self) -> CNF13_W<22> {
        CNF13_W::new(self)
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> MODE14_W<24> {
        MODE14_W::new(self)
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf14(&mut self) -> CNF14_W<26> {
        CNF14_W::new(self)
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> MODE15_W<28> {
        MODE15_W::new(self)
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn cnf15(&mut self) -> CNF15_W<30> {
        CNF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register high (GPIOn_CRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](index.html) module"]
pub struct CRH_SPEC;
impl crate::RegisterSpec for CRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crh::R](R) reader structure"]
impl crate::Readable for CRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crh::W](W) writer structure"]
impl crate::Writable for CRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRH to value 0x4444_4444"]
impl crate::Resettable for CRH_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
